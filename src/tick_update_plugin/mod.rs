pub use super::*;
use iyes_loopless::prelude::*;

mod movement;
pub use movement::*;

mod any_update;
pub use any_update::*;

pub struct TickUpdatePlugin;

impl Plugin for TickUpdatePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(KeyEvent::None)
            .add_system(set_key_event.label("set key event"))
            .add_system(
                player_movement
                .run_if(movement_event_happened)
                .after("set key event")
                .before("grid update")
            ).add_system(
                update_player_movements
                .run_if(any_update_happened)
                .after("set key event")
                .before("grid update")
            ).add_system(
                update_to_grid
                .run_if(any_update_happened)
                .label("grid update")
            ).add_system(
                update_ticks
                .run_if(any_update_happened)
            )
        ;
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

pub fn update_to_grid(
    mut commands: Commands,
    mut entities_query: Query<(Entity, &mut Transform, &mut GridCoords, &mut GridEntityInfo)>,
    grid: Res<Grid>
) {
    for (
        bevy_entity, 
        mut transform, 
        mut coords, 
        mut grid_entity_info
    ) in entities_query.iter_mut() {
        if let Some((corner, _)) = grid.get_entity(&*grid_entity_info) {
            grid.set_depth_of(&mut grid_entity_info);
        
            coords.x = (corner.0 + grid_entity_info.pos.0) as i32;
            coords.y = (corner.1 + grid_entity_info.pos.1) as i32;
    
            update_transform(&coords, &mut transform);
        } else {
            // deletes entities that are no longer in grid
            commands.entity(bevy_entity).despawn_recursive();
        }
    };
}

pub fn update_transform(coords: &GridCoords, transform: &mut Transform) {
    transform.translation.x = coords.x as f32 * 256.0 + 128.0;
    transform.translation.y = coords.y as f32 * 256.0 + 128.0;
}
