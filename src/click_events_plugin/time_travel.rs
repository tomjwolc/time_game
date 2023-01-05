use super::*;

// !!Assumptions!!
// - If an entity is on the inside of the time machine, no part of it will be on the outside
pub fn enable_activate_time_machine(
    // mut commands: Commands,
    mut grid: ResMut<Grid>,
    ticks: Res<Ticks>,
    clicked_time_machine: Res<ClickedTimeMachine>,
) {
    let time_machine_info = clicked_time_machine.0.as_ref().unwrap();
    let contains_player = grid.tm_contains_index(
        &grid.get_entity(time_machine_info).unwrap().1, 
        grid.get_entity_index_from_id("Player", 0).unwrap()
    );
    let grid_clone = grid.clone();
    let (
        start_instance, 
        id 
    ) = if let Some((_, GridEntity::TimeMachine { 
        start_instance,
        id,
        ..
    })) = grid.get_entity_mut(time_machine_info) {
        ( start_instance, id )
    } else {
        panic!("Could not find the clicked time machine in the current grid!?!?!?!?");
    };

    if start_instance.is_none() {
        *start_instance = Some((ticks.0, None, grid_clone));
    } else if !contains_player {
        *start_instance = None;

        grid.remove_entity(time_machine_info);
    } else {
        let (start, end, instance_grid) = start_instance.as_mut().unwrap();

        *end = Some(ticks.0);

        instance_grid.replace_time_machine(
            grid_clone, 
            instance_grid.get_entity_index_from_id("TimeMachine", *id).unwrap(), 
            *start
        );

        *grid = instance_grid.clone();
    }
}