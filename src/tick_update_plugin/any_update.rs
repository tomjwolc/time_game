use super::*;

pub fn any_update_happened(
    key_event: Res<KeyEvent>
) -> bool {
    key_event.to_ivec().is_some()
}

pub fn update_player_movements(
    mut grid: ResMut<Grid>,
    player_info_query: Query<&GridEntityInfo, With<Player>>,
    key_event: Res<KeyEvent>
) {
    let player_entity = &mut grid.get_entity_mut(player_info_query.single()).unwrap().1;

    if let GridEntity::Player { movements } = player_entity {
        movements.push(key_event.to_ivec().unwrap());
    }
}

pub fn update_ticks(mut ticks: ResMut<Ticks>) {
    ticks.0 += 1;
}

pub fn update_events(
    mut grid: ResMut<Grid>,
    ticks: ResMut<Ticks>,
    clicked: ResMut<ClickedTimeMachine>
) {
    grid.update_events(ticks.0, clicked)
}