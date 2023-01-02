use super::*;
use iyes_loopless::prelude::*;
use std::collections::HashMap;

// Until I figure out how to make the setup fns only activate once
#[derive(Resource)]
struct LevelSetupCompleted(bool);

pub struct LevelSetupPlugin;

impl Plugin for LevelSetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(LevelSetupCompleted(false))
            .add_system(set_dims.run_if(level_setup_ready).before("pre_grid_setup"))
            .add_system_set(
                ConditionSet::new()
                .run_if(level_setup_ready)
                .before("end_level_setup")
                .label("pre_grid_setup")
                .with_system(setup_time_machine_parts)
                .with_system(resize_level)
                .into()
            )/*.add_system(
                setup_time_machine_grid
                .run_if(level_setup_ready)
                .after("pre_grid_setup")
            )*/.add_system(
                setup_grid
                .run_if(level_setup_ready)
                .after("pre_grid_setup")
                .before("end_level_setup")
                .label("grid_setup")
            )
            .add_system(end_level_setup.run_if(level_setup_ready).label("end_level_setup"))
        ;
    }
}

fn level_setup_ready(
    ldtk_level_query: Query<&Handle<LdtkLevel>>, 
    level_setup_completed: Res<LevelSetupCompleted>
) -> bool {
    ldtk_level_query.get_single().is_ok() && !level_setup_completed.0
}

fn set_dims(
    ldtk_level: Res<Assets<LdtkLevel>>,
    ldtk_level_query: Query<&Handle<LdtkLevel>>,
    mut dims: ResMut<Dims>
) {
    let level = &ldtk_level.get(ldtk_level_query.single()).unwrap().level;

    dims.x = (level.px_wid / 256) as usize;
    dims.y = (level.px_hei / 256) as usize;
}

fn setup_time_machine_parts(
    mut time_machine_parts_query: Query<(
        &mut GridEntityInfo, 
        &TimeMachinePartType, 
        &GridCoords
    )>,
    dims: Res<Dims>
) {
    let mut time_machine_part_grid: Vec<Vec<Option<(usize, TimeMachinePartType)>>> = vec![vec![None; dims.y]; dims.x];

    for (_, part_type, coords) in time_machine_parts_query.iter() {
        time_machine_part_grid[coords.x as usize][coords.y as usize] = Some((
            0,
            *part_type
        ));
    }

    let mut id = 1;

    let mut sorted_time_machine_parts: Vec<(
        Mut<GridEntityInfo>, 
        &TimeMachinePartType, 
        &GridCoords
    )> = time_machine_parts_query.iter_mut().collect();

    sorted_time_machine_parts.sort_by(|(_, _, coords1), (_, _, coords2)| {
        match coords1.x.partial_cmp(&coords2.x).unwrap() {
            std::cmp::Ordering::Equal => coords1.y.partial_cmp(&coords2.y).unwrap(),
            ordering => ordering 
        }
    });

    let mut corner_map: HashMap<usize, GridCoords> = HashMap::new();

    for (mut grid_entity, _, coords) in sorted_time_machine_parts {
        if time_machine_part_grid[coords.x as usize][coords.y as usize].unwrap().0 == 0 {
            expand_connections(coords.x as usize, coords.y as usize, &mut time_machine_part_grid, id);
            corner_map.insert(id, *coords);

            id += 1;
        }

        grid_entity.id = time_machine_part_grid[coords.x as usize][coords.y as usize].unwrap().0;
        let corner = corner_map.get(&grid_entity.id).unwrap();
        grid_entity.pos = (
            (coords.x - corner.x) as usize,
            (coords.y - corner.y) as usize
        )
    }
}

fn expand_connections(x: usize, y: usize, grid: &mut Vec<Vec<Option<(usize, TimeMachinePartType)>>>, expanding_id: usize) {
    // println!("[{}]: Expanding connections at ({}, {})", expanding_id, x, y);

    if let Some((id, part_type)) = grid[x][y] { if id == 0 {
        grid[x][y].as_mut().unwrap().0 = expanding_id;

        // println!("[{}]: {:?}", expanding_id, grid[x][y]);

        if x > 0 && grid[ x-1 ][ y ].is_some() && part_type.fits_on_left(&grid[ x-1 ][ y ].unwrap().1) {
            expand_connections(x - 1, y, grid, expanding_id);
        }

        if x < grid.len() - 1 && grid[ x+1 ][ y ].is_some() && part_type.fits_on_right(&grid[ x+1 ][ y ].unwrap().1) {
            expand_connections(x + 1, y, grid, expanding_id);
        }

        if y > 0 && grid[ x ][ y-1 ].is_some() && part_type.fits_on_bottom(&grid[ x ][ y-1 ].unwrap().1) {
            expand_connections(x, y - 1, grid, expanding_id);
        }

        if y < grid[0].len() - 1 && grid[ x ][ y+1 ].is_some() && part_type.fits_on_top(&grid[ x ][ y+1 ].unwrap().1) {
            expand_connections(x, y + 1, grid, expanding_id);
        }
    }}
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

// fn setup_time_machine_grid(
//     dims: Res<Dims>,
//     mut grid: ResMut<Grid>,
//     mut tm_grid_entities: Query<(&mut GridEntityInfo, &GridCoords, &Time)>
// ) {
//     *grid = Grid::new_sized(dims.x, dims.y);

// }

fn setup_grid(
    dims: Res<Dims>,
    mut grid: ResMut<Grid>,
    mut grid_entities: Query<(&mut GridEntityInfo, &GridCoords, Option<&TimeMachinePartType>)>,
) {
    *grid = Grid::new_sized(dims.x, dims.y);

    for (i, (mut grid_entity_info, coords, opt_part_type)) in grid_entities.iter_mut().enumerate() {
        if grid_entity_info.id == 0 { grid_entity_info.id = i };

        grid.add_entity_to_pos(
            coords.x as usize,
            coords.y as usize,
            &grid_entity_info
        );

        let (corner, entity) = grid.get_entity_mut(&grid_entity_info).unwrap();

        if grid_entity_info.pos == (0, 0) {
            *corner = (
                coords.x as usize,
                coords.y as usize
            );
        }

        entity.try_add_part_to_grid(
            grid_entity_info.pos.0,
            grid_entity_info.pos.1,
            opt_part_type
        );
    }

    // println!("{}", *grid);
}

fn end_level_setup(mut level_setup_completed: ResMut<LevelSetupCompleted>) {
    level_setup_completed.0 = true;
}