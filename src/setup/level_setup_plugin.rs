use super::*;
use iyes_loopless::prelude::*;

// Until I figure out how to make the setup fns only activate once
#[derive(Resource)]
struct LevelSetupCompleted(bool);

pub struct LevelSetupPlugin;

impl Plugin for LevelSetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LevelSetupCompleted(false))
            .add_system(end_level_setup.run_if(level_setup_ready).label("end_level_setup"))
            .add_system_set(
                ConditionSet::new()
                .run_if(level_setup_ready)
                .before("end_level_setup")
                .label("pre_grid_setup")
                .with_system(setup_time_machine_parts)
                .with_system(resize_level)
                .into()
            ).add_system(
                setup_entity_grid
                .run_if(level_setup_ready)
                .after("pre_grid_setup")
                .before("end_level_setup")
            )
        ;
    }
}

fn level_setup_ready(
    ldtk_level_query: Query<&Handle<LdtkLevel>>, 
    level_setup_completed: Res<LevelSetupCompleted>
) -> bool {
    ldtk_level_query.get_single().is_ok() && !level_setup_completed.0
}

fn setup_time_machine_parts(
    ldtk_level: Res<Assets<LdtkLevel>>,
    ldtk_level_query: Query<&Handle<LdtkLevel>>,
    mut time_machine_parts_query: Query<(&mut TimeMachinePartType, &mut TimeMachineId)>
) {
    let level = &ldtk_level.get(ldtk_level_query.single()).unwrap().level;
    let ldtk_time_machine_entities = &level.layer_instances.as_ref().expect("")[1].entity_instances;

    for (i, (mut part_type, mut id)) in time_machine_parts_query.iter_mut().enumerate() {
        let enum_varient_string = ldtk_time_machine_entities[i]
            .field_instances[0]
            .real_editor_values[0]
            .as_ref().unwrap()
            .get("params").unwrap()
            .get(0).unwrap()
            .as_str().unwrap();

        *part_type = enum_varient_string.parse().unwrap();
        id.0 = 0; // updated correctly after setup entity grid
    }
}

fn resize_level(
    ldtk_level: Res<Assets<LdtkLevel>>,
    mut ldtk_level_query: Query<(&mut Transform, &Handle<LdtkLevel>)>,
    windows: Res<Windows>
) {
    let (mut transform, ldtk_handle) = ldtk_level_query.single_mut();
    let window = windows.get_primary().unwrap();
    let level = &ldtk_level.get(ldtk_handle).unwrap().level;
    let scaling_factor = (window.width() / level.px_wid as f32).min(window.height() / level.px_hei as f32);

    transform.scale *= scaling_factor;
    transform.translation -= scaling_factor * Vec3::new(level.px_wid as f32, level.px_hei as f32, 0.0) / 2.0;
}

fn setup_entity_grid(
    // Assets handle for the LdtkLevel resource
    ldtk_level: Res<Assets<LdtkLevel>>,
    // Query for the LdtkLevel handle
    ldtk_level_query: Query<&Handle<LdtkLevel>>,
    // Mutable reference to the EntityGrid resource
    mut grid: ResMut<EntityGrid>,
    // Query for the GridCoords component of entities with the Player component
    player_query: Query<&GridCoords, With<Player>>,
    // Query for the GridCoords component of entities with the Box component
    box_query: Query<&GridCoords, With<Box>>,
    // Query for the GridCoords and TimeMachinePartType components of entities
    time_machine_parts_query: Query<(&GridCoords, &TimeMachinePartType)>
) {
    // Get the LdtkLevel resource
    let level = &ldtk_level.get(ldtk_level_query.single()).unwrap().level;

    // Initialize the EntityGrid with the dimensions of the LdtkLevel
    grid.0 = vec![vec![GridState::Nothing; (level.px_wid / 256) as usize]; (level.px_hei / 256) as usize];

    // Get the GridCoords of the entity with the Player component
    let player_coords = player_query.single();
    // Update the EntityGrid with the Player's position
    grid.0[player_coords.y as usize][player_coords.x as usize] = GridState::Player;

    // Iterate over the entities with the Box component and update the EntityGrid with their positions
    for box_coords in box_query.iter() {
        grid.0[box_coords.y as usize][box_coords.x as usize] = GridState::Box;
    }

    // Iterate over the entities with the TimeMachinePartType component and update the EntityGrid with their positions
    for (time_machine_coords, part_type) in time_machine_parts_query.iter() {
        grid.0[time_machine_coords.y as usize][time_machine_coords.x as usize] = GridState::TimeMachine(0, *part_type);
    }  

    let mut id = 1;

    for (coords, _) in time_machine_parts_query.iter() {
        if let GridState::TimeMachine(0, _) = grid.0[coords.y as usize][coords.x as usize] {
            expand_connections(coords.y as usize, coords.x as usize, &mut grid.0, id);

            id += 1;
        }
    }

    print_vector_of_vectors(grid.0.clone());
}
use std::fmt::Debug;

fn print_vector_of_vectors<T: Debug>(vv: Vec<Vec<T>>) {
    println!("[");
    for v in vv {
        println!("    {v:?},");
    }
    println!("]");
}

fn expand_connections(x: usize, y: usize, grid: &mut Vec<Vec<GridState>>, expanding_id: usize) {
    if let GridState::TimeMachine(id, _) = &mut grid[x][y] { *id = expanding_id; }

    if let GridState::TimeMachine(id, part_type) = grid[x][y] {
        if id == 0 {
            if x > 0 && grid[x - 1][y].is_tm() && part_type.fits_on_right(grid[x - 1][y].to_tm().unwrap().1) {
                expand_connections(x - 1, y, grid, expanding_id);
            }

            if x < grid[0].len() && grid[x + 1][y].is_tm() && part_type.fits_on_left(grid[x + 1][y].to_tm().unwrap().1)  {
                expand_connections(x + 1, y, grid, expanding_id);
            }

            if y > 0 && grid[x][y - 1].is_tm() && part_type.fits_on_top(grid[x][y - 1].to_tm().unwrap().1)  {
                expand_connections(x, y - 1, grid, expanding_id);
            }

            if y < grid.len() && grid[x][y + 1].is_tm() && part_type.fits_on_bottom(grid[x][y + 1].to_tm().unwrap().1)  {
                expand_connections(x, y + 1, grid, expanding_id);
            }
        }
    }
}

fn end_level_setup(mut level_setup_completed: ResMut<LevelSetupCompleted>) {
    level_setup_completed.0 = true;
}