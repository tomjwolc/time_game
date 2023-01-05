use super::*;

pub fn any_update_happened(
    key_event: Res<KeyEvent>
) -> bool {
    key_event.to_ivec().is_some()
}

pub fn update_movements(
    mut grid: ResMut<Grid>,
    key_event: Res<KeyEvent>
) {
    grid.add_movement(key_event.to_ivec().unwrap());
}

pub fn update_ticks(mut ticks: ResMut<Ticks>) {
    ticks.0 += 1;
}

pub fn update_events(
    mut grid: ResMut<Grid>,
    ticks: ResMut<Ticks>,
    mut clicked: ResMut<ClickedTimeMachine>
) {
    grid.update_events(ticks.0, &mut clicked);
    // println!("update events: {}, clicked: {:?}", *grid, clicked.0);
}