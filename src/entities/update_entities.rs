use super::super::*;

pub fn update_to_grid(
    mut entities_query: Query<(&mut Transform, &mut GridCoords, &GridEntityInfo)>,
    grid: Res<Grid>
) {
    for (transform, coords, grid_entity_info) in entities_query.iter_mut() {
        let grid_entity = grid
            .get_entity(grid_entity_info)
            .expect(format!("Could not find the entity: {:?}", grid_entity_info).as_str());
        
        


    };
}