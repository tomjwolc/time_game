use bevy_ecs_ldtk::GridCoords;

use super::*;

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

// pub fn try_move(
//     grid: &mut Grid,
//     coords: GridCoords,
//     direction: Direction
// ) -> bool {
//     let next_tile = &mut match (direction, coords.x as usize, coords.y as usize) {
//         (Direction::Up, x, y) if y > 0 => grid.entity_grid[ x ][ y-1 ],
//         (Direction::Down, x, y) if y < grid.entity_grid[0].len() - 1 => grid.entity_grid[ x ][ y+1 ],
//         (Direction::Left, x, y) if x < grid.entity_grid.len() - 1 => grid.entity_grid[ x+1 ][ y ],
//         (Direction::Right, x, y) if x > 0 => grid.entity_grid[ x-1 ][ y ],
//         (_, x, y) => grid.entity_grid[ x ][ y ]
//     };



//     *next_tile == 0
// }

/* 
pub fn player_movement(
    mut input: ResMut<Input<KeyCode>>,
    mut query: Query<(&mut GridCoords, &mut Transform), With<Player>>,
    grid: Res<Grid>
) {
    if let Ok((mut coords, mut transform)) = query.get_single_mut() {
        if 
            input.just_pressed(KeyCode::W) && 
            can_move_up(coords.x as usize, coords.y as usize, &grid.0) 
        {
            input.clear_just_pressed(KeyCode::W);
    
            coords.y += 1;
        } else if 
            input.just_pressed(KeyCode::A) && 
            can_move_left(coords.x as usize, coords.y as usize, &grid.0) 
        {
            input.clear_just_pressed(KeyCode::A);
    
            coords.x -= 1;
        } else if 
            input.just_pressed(KeyCode::S) && 
            can_move_down(coords.x as usize, coords.y as usize, &grid.0) 
        {
            input.clear_just_pressed(KeyCode::S);
    
            coords.y -= 1;
        }  else if 
            input.just_pressed(KeyCode::D) && 
            can_move_right(coords.x as usize, coords.y as usize, &grid.0) 
        {
            input.clear_just_pressed(KeyCode::D);
    
            coords.x += 1;
        }

        update_transform(&coords, &mut transform);
    }
}

fn can_move_up(x: usize, y: usize, entity_grid: &Vec<Vec<GridState>>) -> bool {
    y < entity_grid[0].len() - 1 && (
        !entity_grid[x][y].is_tm() || 
        entity_grid[x][y].to_tm().unwrap().1.can_enter_exit_top()
    ) && (
        !entity_grid[x][y + 1].is_tm() || 
        entity_grid[x][y + 1].to_tm().unwrap().1.can_enter_exit_bottom()
    )
}

fn can_move_down(x: usize, y: usize, entity_grid: &Vec<Vec<GridState>>) -> bool {
    y > 0 && (
        !entity_grid[x][y].is_tm() || 
        entity_grid[x][y].to_tm().unwrap().1.can_enter_exit_bottom()
    ) && (
        !entity_grid[x][y - 1].is_tm() || 
        entity_grid[x][y - 1].to_tm().unwrap().1.can_enter_exit_top()
    )
}

fn can_move_left(x: usize, y: usize, entity_grid: &Vec<Vec<GridState>>) -> bool {
    x > 0 && (
        !entity_grid[x][y].is_tm() || 
        entity_grid[x][y].to_tm().unwrap().1.can_enter_exit_left()
    ) && (
        !entity_grid[x - 1][y].is_tm() || 
        entity_grid[x - 1][y].to_tm().unwrap().1.can_enter_exit_right()
    )
}

fn can_move_right(x: usize, y: usize, entity_grid: &Vec<Vec<GridState>>) -> bool {
    x < entity_grid.len() - 1 && (
        !entity_grid[x][y].is_tm() || 
        entity_grid[x][y].to_tm().unwrap().1.can_enter_exit_right()
    ) && (
        !entity_grid[x + 1][y].is_tm() || 
        entity_grid[x + 1][y].to_tm().unwrap().1.can_enter_exit_left()
    )
}
*/