use super::*;

// !!Assumptions!!
// - If an entity is on the inside of the time machine, no part of it will be on the outside
fn enable_activate_time_machine(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    ticks: Res<Ticks>,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    time_machine_info_query: Query<&GridEntityInfo, With<TimeMachine>>
) {
    if !buttons.just_pressed(MouseButton::Left) { return; }

    let cursor_pos = windows.get_primary().unwrap().cursor_position().unwrap_or(Vec2::new(-1.0, -1.0));

    // Gets the time machine that has been clicked with the most time machine depth
    let clicked_time_machine: Option<&GridEntityInfo> = time_machine_info_query.into_iter()
        .filter(|grid_entity_info| grid_entity_info.pos == (0, 0))
        .min_by_key(|grid_entity_info| {
            if let Some((
                corner, 
                GridEntity::TimeMachine { grid: tm_grid, .. }
            )) = grid.get_entity(grid_entity_info) {
                let (x1, y1, x2, y2) = (
                    corner.0 as f32,
                    corner.1 as f32,
                    (corner.0 + tm_grid.len()) as f32,
                    (corner.1 + tm_grid[0].len()) as f32
                );

                if x1 < cursor_pos.x && cursor_pos.x < x2 && y1 < cursor_pos.y && cursor_pos.y < y2 {
                    grid_entity_info.time_machine_depth as i32
                } else {
                    -1
                }
            } else {
                -1
            }
        });
    
    // if the click clicked a time machine
    if let Some(time_machine_info) = clicked_time_machine {
        let grid_clone = grid.clone();

        // if the time machine info points to a time machine in the the current grid (ALWAYS TRUE)
        if let Some((_, GridEntity::TimeMachine { 
            start_instance, 
            grid: tm_grid,
            id,
            ..
        })) = &mut grid.get_entity_mut(time_machine_info) {
            // If the clicked time machine has a start instance
            if let Some((start, end, mut instance_grid)) = start_instance.clone() {

                // If the start instance has the time machine that was clicked (ALWAYS TRUE)
                if let Some((_, GridEntity::TimeMachine { 
                    grid: instance_tm_grid, 
                    .. 
                })) = instance_grid.get_entity_from_id_mut("TimeMachine", *id) {


                } else { 
                    panic!("Could not find the time machine in the instance it was instantiated!?!?!?!?");
                }
            } else {
                *start_instance = Some((ticks.0, None, grid_clone));
            }
        } else {
            panic!("Could not find the clicked time machine in the current grid!?!?!?!?");
        }
    }
}