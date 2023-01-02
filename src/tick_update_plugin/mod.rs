pub use super::*;
use iyes_loopless::prelude::*;

mod movement;
pub use movement::*;

pub struct TickUpdatePlugin;

impl Plugin for TickUpdatePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(KeyEvent::None)
            .add_system(set_key_event.label("set_key_event"))
            .add_system(
                player_movement
                .run_if(movement_event_happened)
                .after("set_key_event")
                .label("key_event")
            )
            .add_system(
                update_to_grid
                .run_if(movement_event_happened)
                .after("key_event")
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
    mut entities_query: Query<(&mut Transform, &mut GridCoords, &GridEntityInfo)>,
    grid: Res<Grid>
) {
    for (mut transform, mut coords, grid_entity_info) in entities_query.iter_mut() {
        let (corner, _) = grid
            .get_entity(grid_entity_info)
            .expect(format!("Could not find the entity: {:?}", grid_entity_info).as_str());
        
        coords.x = (corner.0 + grid_entity_info.pos.0) as i32;
        coords.y = (corner.1 + grid_entity_info.pos.1) as i32;

        update_transform(&coords, &mut transform);
    };
}

pub fn update_transform(coords: &GridCoords, transform: &mut Transform) {
    transform.translation.x = coords.x as f32 * 256.0 + 128.0;
    transform.translation.y = coords.y as f32 * 256.0 + 128.0;
}
