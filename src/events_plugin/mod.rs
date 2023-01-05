pub use super::*;
use iyes_loopless::prelude::*;

mod time_travel;
pub use time_travel::*;

mod movement;
pub use movement::*;

mod any_update;
pub use any_update::*;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(KeyEvent::None)
            .insert_resource(ClickedTimeMachine(None))
            .add_system(set_key_event.label("set key event"))
            .add_system(set_clicked.label("set clicked"))

            .add_system(
                update_ticks
                .run_if(any_update_happened)
                .label("update ticks")
                .after("set key event")
                .after("set clicked")
            ).add_system(
                player_movement
                .run_if(movement_event_happened)
                .label("player movement")
                .after("update ticks")
            ).add_system(
                update_events
                .run_if(any_update_happened)
                .label("update events")
                .after("player movement")
            ).add_system(
                enable_activate_time_machine
                .run_if(time_machine_has_been_clicked)
                .label("enable/activate tm")
                .after("update events")
            ).add_system(
                update_movements
                .run_if(any_update_happened)
                .label("update movements")
                .after("enable/activate tm")
            ).add_system(
                print_grid
                .run_if(anything_happened)
                .after("update movements")
                .before("update to grid")
            ).add_system(
                update_to_grid
                .run_if(anything_happened)
                .label("update to grid")
                .after("update movements")
            ).add_system(
                reset_clicked
                .run_if(anything_happened)
                .label("reset clicked")
                .after("update to grid")
            )
        ;
    }
}

fn print_grid(grid: Res<Grid>) { println!("print_grid: {}", *grid); }

#[derive(Resource)]
pub struct ClickedTimeMachine(pub Option<GridEntityInfo>);

fn set_clicked(
    grid: Res<Grid>,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    mut clicked: ResMut<ClickedTimeMachine>,
    time_machine_info_query: Query<&GridEntityInfo, With<TimeMachine>>,
    level_window_info: Res<LevelWindowInfo>
) {
    if buttons.just_pressed(MouseButton::Left) {
        let cursor_pos = windows.get_primary().unwrap().cursor_position().unwrap_or(Vec2::new(-1.0, -1.0));

        // Gets the time machine that has been clicked with the most time machine depth
        clicked.0 = time_machine_info_query.into_iter()
        .filter(|grid_entity_info| {
            grid_entity_info.pos == (0, 0) && (if let Some((
                corner, 
                GridEntity::TimeMachine { grid: tm_grid, .. }
            )) = grid.get_entity(grid_entity_info) {
                let (x1, y1, x2, y2) = (
                    level_window_info.scaling_factor * (TILE_SIZE * corner.0) as f32 + level_window_info.offset.0,
                    level_window_info.scaling_factor * (TILE_SIZE * corner.1) as f32 + level_window_info.offset.1,
                    level_window_info.scaling_factor * (TILE_SIZE * (corner.0 + tm_grid.len())) as f32 + level_window_info.offset.0,
                    level_window_info.scaling_factor * (TILE_SIZE * (corner.1 + tm_grid[0].len())) as f32 + level_window_info.offset.1
                );

                println!("{:?}  ---  {:?}", (x1, y1, x2, y2), cursor_pos);

                if x1 < cursor_pos.x && cursor_pos.x < x2 && y1 < cursor_pos.y && cursor_pos.y < y2 {
                    true
                } else {
                    false
                }
            } else {
                false
            })
        })
        .max_by_key(|grid_entity_info| {
            grid_entity_info.time_machine_depth
        }).cloned();
    }
}

#[derive(Resource)]
pub enum KeyEvent {
    W,
    A,
    S,
    D,
    Space,
    None
}

impl KeyEvent {
    fn to_direction(&self) -> Option<MoveDirection> {
        match self {
            KeyEvent::W => Some(MoveDirection::Up),
            KeyEvent::A => Some(MoveDirection::Left),
            KeyEvent::S => Some(MoveDirection::Down),
            KeyEvent::D => Some(MoveDirection::Right),
            KeyEvent::Space => None,
            KeyEvent::None => None,
        }
    }
    
    pub fn to_ivec(&self) -> Option<IVec2> {
        match self {
            Self::W => Some(IVec2::new(0, 1)),
            Self::A => Some(IVec2::new(-1, 0)),
            Self::S => Some(IVec2::new(0, -1)),
            Self::D => Some(IVec2::new(1, 0)),
            Self::Space => Some(IVec2::ZERO),
            Self::None => None
        }
    }
}

pub fn set_key_event(
    mut input: ResMut<Input<KeyCode>>,
    mut key_event: ResMut<KeyEvent>
) {
    *key_event = if input.just_pressed(KeyCode::W) {
        input.clear_just_pressed(KeyCode::W);
        KeyEvent::W

    } else if input.just_pressed(KeyCode::A) {
        input.clear_just_pressed(KeyCode::A);
        KeyEvent::A

    } else if input.just_pressed(KeyCode::S) {
        input.clear_just_pressed(KeyCode::S);
        KeyEvent::S

    }  else if input.just_pressed(KeyCode::D) {
        input.clear_just_pressed(KeyCode::D);
        KeyEvent::D

    } else if input.just_pressed(KeyCode::Space) {
        input.clear_just_pressed(KeyCode::Space);
        KeyEvent::Space

    } else {
        KeyEvent::None
    }
}

fn time_machine_has_been_clicked(clicked: Res<ClickedTimeMachine>) -> bool {
    clicked.0.is_some()
}

fn anything_happened(clicked: Res<ClickedTimeMachine>, key_event: Res<KeyEvent>) -> bool {
    time_machine_has_been_clicked(clicked) || any_update_happened(key_event)
}

fn reset_clicked(
    mut clicked: ResMut<ClickedTimeMachine>
) {
    clicked.0 = None;
}

pub fn update_to_grid(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    ldtk_level: Query<Entity, With<Handle<LdtkLevel>>>,
    mut entities_query: Query<(Entity, &mut TextureAtlasSprite, &mut Transform, &mut GridCoords, &mut GridEntityInfo)>,
    grid: Res<Grid>
) {
    let mut grid_entities: Vec<&((usize, usize), GridEntity)> = grid
        .entities_iter()
        .filter(|(_, grid_entity)| grid_entity != &GridEntity::None)
        .collect();

    for (
        bevy_entity, 
        mut texture,
        mut transform, 
        mut coords, 
        mut grid_entity_info
    ) in entities_query.iter_mut() {
        if let Some((corner, entity)) = grid.get_entity(&*grid_entity_info) {
            if let GridEntity::TimeMachine { start_instance: Some(_), .. } = entity {
                texture.color = Color::RED;
            } else {
                texture.color = Color::WHITE;
            }

            grid.set_depth_of(&mut grid_entity_info);
        
            coords.x = (corner.0 + grid_entity_info.pos.0) as i32;
            coords.y = (corner.1 + grid_entity_info.pos.1) as i32;
    
            update_transform(&coords, &mut transform);

            // Removes the particular entity from the vec
            grid_entities = grid_entities
                .into_iter()
                .filter(|(_, grid_entity)| {
                    !match (grid_entity, grid_entity_info.variant) {
                        (GridEntity::Player { .. }, "Player") => true,
                        (GridEntity::PastPlayer { id: entity_id, .. }, "PastPlayer") |
                        (GridEntity::Box {id: entity_id, .. }, "Box") |
                        (GridEntity::TimeMachine { id: entity_id, .. }, "TimeMachine") => *entity_id == grid_entity_info.id,
                        _ => false
                    }
                }).collect();
        } else {
            // deletes entities that are no longer in grid
            commands.entity(bevy_entity).despawn_recursive();
        }
    };

    for (corner, grid_entity) in grid_entities {
        grid_entity.spawn_bundle(
            *corner, 
            &mut commands, 
            &mut texture_atlases, 
            &asset_server,
            ldtk_level.single()
        );
    }
}

pub fn update_transform(coords: &GridCoords, transform: &mut Transform) {
    transform.translation.x = coords.x as f32 * 256.0 + 128.0;
    transform.translation.y = coords.y as f32 * 256.0 + 128.0;
}
