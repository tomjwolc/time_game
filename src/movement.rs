use bevy_ecs_ldtk::GridCoords;

use super::*;

pub fn player_movement(
    mut input: ResMut<Input<KeyCode>>,
    mut query: Query<(&mut GridCoords, &mut Transform), With<Player>>,
) {
    if let Ok((mut coords, mut transform)) = query.get_single_mut() {
        if input.just_pressed(KeyCode::W) {
            input.clear_just_pressed(KeyCode::W);
    
            coords.y += 1;
        } else if input.just_pressed(KeyCode::A) {
            input.clear_just_pressed(KeyCode::A);
    
            coords.x -= 1;
        } else if input.just_pressed(KeyCode::S) {
            input.clear_just_pressed(KeyCode::S);
    
            coords.y -= 1;
        } else if input.just_pressed(KeyCode::D) {
            input.clear_just_pressed(KeyCode::D);
    
            coords.x += 1;
        }

        update_transform(&coords, &mut transform);
    }
}