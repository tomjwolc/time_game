pub use super::*;
use iyes_loopless::prelude::*;

mod time_travel;
pub use time_travel::*;

pub struct ClickEventsPlugin;

impl Plugin for ClickEventsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClickedTimeMachine(None))
            .add_system(set_clicked.label("set clicked"))
            .add_system(
                enable_activate_time_machine
                .run_if(time_machine_has_been_clicked)
                .before("update grid")
                .after("set clicked")
            )
            .add_system(
                update_to_grid
                .run_if(time_machine_has_been_clicked)
                .label("update grid")
            )
        ;
    }
}

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
    if !buttons.just_pressed(MouseButton::Left) { 
        clicked.0 = None;
    } else {
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

        println!("clicked: {:?}", clicked.0);
    }
}

fn time_machine_has_been_clicked(clicked: Res<ClickedTimeMachine>) -> bool {
    clicked.0.is_some()
}