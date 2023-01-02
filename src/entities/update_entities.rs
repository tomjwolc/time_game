use super::super::*;

pub fn update_to_grid(
    mut entities_query: Query<(&mut Transform, &mut GridCoords, &GridEntityInfo)>,
    grid: Res<Grid>
) {
    for (mut transform, mut coords, grid_entity_info) in entities_query.iter_mut() {
        let (corner, _) = grid
            .get_entity(grid_entity_info)
            .expect(format!("Could not find the entity: {:?}", grid_entity_info).as_str());
        
        coords.x = (corner.0 + grid_entity_info.pos.0) as i32;
        coords.y = (corner.1 + grid_entity_info.pos.1) as i32;

        update_transform(&coords, &mut transform);
    };
}

pub fn update_transform(coords: &GridCoords, transform: &mut Transform) {
    transform.translation.x = coords.x as f32 * 256.0 + 128.0;
    transform.translation.y = coords.y as f32 * 256.0 + 128.0;
}