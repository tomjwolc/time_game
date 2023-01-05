use super::*;

#[derive(Default, Debug, PartialEq, Clone)]
pub enum GridEntity {
    Player {
        movements: Vec<IVec2>
    },
    PastPlayer {
        id: usize, 
        movements: Vec<IVec2>
    },
    Box {
        id: usize
    },
    TimeMachine {
        start_instance: Option<(usize, Option<usize>, Option<usize>, Grid)>,
        id: usize, 
        grid: Vec<Vec<(TimeMachinePartType, usize)>>
    },
    #[default]
    None
}

impl GridEntity {
    fn is_tm(&self) -> bool {
        if let GridEntity::TimeMachine { .. } = self { true } else { false }
    }
        
    fn get_tm_grid<'a>(&'a self) -> &'a Vec<Vec<(TimeMachinePartType, usize)>> {
        if let GridEntity::TimeMachine{ grid, .. } = self {
            grid
        } else {
            panic!("Tried to get tm_grid entity, but it wasn't the time machine varient")
        }
    }
    
    fn get_tm_grid_mut<'a>(&'a mut self) -> &'a mut Vec<Vec<(TimeMachinePartType, usize)>> {
        if let GridEntity::TimeMachine{ grid, .. } = self {
            grid
        } else {
            panic!("Tried to get tm_grid entity, but it wasn't the time machine varient")
        }
    }

    fn is_pushable(&self) -> bool {
        match self {
            GridEntity::TimeMachine { .. } => false,
            GridEntity::None { .. } => false,
            _ => true
        }
    }

    fn entity_at<'a>(&'a self, corner: (usize, usize), pos: (usize, usize), entities: &'a Vec<((usize, usize), GridEntity)>) -> &'a GridEntity {
        match self {
            GridEntity::TimeMachine { grid, .. } => {
                let entity = &entities[grid[ pos.0 - corner.0 ][ pos.1 - corner.1 ].1];

                entity.1.entity_at(entity.0, pos, entities)
            },
            entity => entity
        }
    }

    fn entity_index_at(
        &self, 
        corner: (usize, usize), 
        pos: (usize, usize), 
        entities: &Vec<((usize, usize), GridEntity)>
    ) -> usize {
        if let GridEntity::TimeMachine { grid, .. } = self {
            let entity = &entities[ grid[ pos.0 - corner.0 ][ pos.1 - corner.1 ].1 ];

            if entity.1.is_tm() {
                entity.1.entity_index_at(
                    entity.0, 
                    pos, 
                    entities
                )
            } else {
                grid[ pos.0 - corner.0 ][ pos.1 - corner.1 ].1
            }
        } else {
            panic!("Called entity_index_at on non-time machine");
        }
    }

    fn set_to_pos(
        self_index: usize, 
        corner: (usize, usize), 
        pos: (usize, usize), 
        entity_index: usize, 
        entities: &mut Vec<((usize, usize), GridEntity)>
    ) {
        let grid = entities[ self_index ].1.get_tm_grid();
        let next_index = grid[ pos.0 - corner.0 ][ pos.1 - corner.1 ].1;

        if entities[ next_index ].1.is_tm() {
            GridEntity::set_to_pos(next_index, corner, pos, entity_index, entities);
        } else {
            let grid = entities[ self_index ].1.get_tm_grid_mut();
            grid[ pos.0 - corner.0 ][ pos.1 - corner.1 ].1 = entity_index;
        }
    }

    pub fn try_add_part_to_grid(&mut self, x: usize, y: usize, opt_part_type: Option<&TimeMachinePartType>) {
        match (self, opt_part_type) {
            (
                GridEntity::TimeMachine { grid, .. }, 
                Some(part_type)
            ) => {
                while grid.len() <= x {
                    grid.push(Vec::new());
                }
    
                while grid[ x ].len() <= y {
                    grid[ x ].push((TimeMachinePartType::Middle, 0));
                }
    
                grid[ x ][ y ] = (
                    *part_type,
                    0
                );
            },
           _ => {}
        }
    }

    fn add_to_grid(
        entity_index: usize, 
        add_index: usize, 
        pos: (usize, usize), 
        entities: &mut Vec<((usize, usize), GridEntity)>
    ) {
        match &mut entities[entity_index] {
            (corner, GridEntity::TimeMachine { grid, .. }) => {
                let x = pos.0 - corner.0;
                let y = pos.1 - corner.1;
                
                if grid[ x ][ y ].1 == 0 {
                    grid[ x ][ y ].1 = add_index;
                } else {
                    GridEntity::add_to_grid(
                        entity_index,
                        add_index,
                        pos,
                        entities
                    )
                }
            },
            (_, entity) => panic!("Can't add to the variant: {:?}", entity)
        }
    }

    fn get_depth_of(
        &self, 
        corner: (usize, usize), 
        pos: (usize, usize), 
        index: usize,
        entities: &Vec<((usize, usize), GridEntity)>
    ) -> usize {
        match self {
            GridEntity::TimeMachine { grid, .. } => {
                let entity = &entities[ grid[ pos.0 - corner.0 ][ pos.1 - corner.1 ].1 ];

                entity.1.get_depth_of(entity.0, pos, index, entities)
            },
            _ => 0
        }
    }

    fn get_contents<'a>(
        &self,
        entities: &'a Vec<((usize, usize), GridEntity)>
    ) -> Vec<usize> {
        match self {
            GridEntity::TimeMachine { grid, .. } => {
                let mut contents: Vec<usize> = Vec::new();

                for row in grid {
                    for (_, index) in row {
                        if entities[ *index ].1 != GridEntity::None && !contents.contains( index ) {
                            contents.push( *index );
                            contents = [
                                contents,
                                entities[ *index ].1.get_contents(entities)
                            ].concat();
                        }
                    }
                }

                contents
            },
            _ => Vec::new()
        }
    }

    fn change_all_indeces(my_index: usize, from_index: usize, to_index: usize, entities: &mut Vec<((usize, usize), GridEntity)>) {
        let sizes = if let GridEntity::TimeMachine { grid, .. } = &entities[ my_index ].1 {
            (grid.len(), grid[0].len())
        } else {
            (0, 0)
        };

        if let GridEntity::TimeMachine { .. } = entities[ my_index ].1 {
            for i in 0..sizes.0 {
                for j in 0..sizes.1 {
                    let index = entities[ my_index ].1.get_tm_grid()[i][j].1;
                    
                    if index == from_index {
                        entities[ my_index ].1.get_tm_grid_mut()[i][j].1 = to_index
                    } else if index != to_index {
                        GridEntity::change_all_indeces(
                            index,
                            from_index, 
                            to_index, 
                            entities
                        );
                    }
                }
            }
        }
    }

    fn change_id(&mut self, new_id: usize) {
        match self {
            GridEntity::PastPlayer { id, .. } |
            GridEntity::Box { id } |
            GridEntity::TimeMachine { id, .. } => *id = new_id,
            _ => {}
        }
    }

    pub fn contains_index(&self, search_index: usize, entities: &Vec<((usize, usize), GridEntity)>) -> bool {
        if let GridEntity::TimeMachine { grid, .. } = self {
            let mut contains = false;

            for row in grid {
                for (_, index) in row {
                    contains = contains || 
                    *index == search_index ||
                    entities[ *index ].1.contains_index(search_index, entities);
                }
            }

            contains
        } else {
            false
        }
    }

    pub fn spawn_bundle(
        &self, 
        corner: (usize, usize), 
        commands: &mut Commands, 
        text_atlases: &mut Assets<TextureAtlas>, 
        asset_server: &Res<AssetServer>,
        bevy_level_entity: Entity
    ) {
        match self {
            GridEntity::PastPlayer { id, .. } => {
                commands.entity(bevy_level_entity).with_children(|parent| {
                    parent.spawn(PastPlayerBundle {
                        component: PastPlayer,
                        position: GridCoords::new(corner.0 as i32, corner.1 as i32),
                        grid_entity: GridEntityInfo {
                            variant: "PastPlayer",
                            id: *id,
                            ..Default::default()
                        },
                        sprite_bundle: SpriteSheetBundle { 
                            sprite: TextureAtlasSprite {
                                index: 50,
                                ..Default::default()
                            },
                            texture_atlas: text_atlases.add(TextureAtlas::from_grid(
                                asset_server.load("tileset_alt.png").into(), 
                                Vec2::new(256.0, 256.0), 
                                8, 
                                8, 
                                None, 
                                None
                            )),
                            transform: Transform::from_xyz(
                                (TILE_SIZE * corner.0 + TILE_SIZE / 2) as f32,
                                (TILE_SIZE * corner.1 + TILE_SIZE / 2) as f32, 
                                0.0
                            ),
                            ..Default::default() 
                        },
                    });
                });
            },
            GridEntity::Box { id, .. } => {
                commands.entity(bevy_level_entity).with_children(|parent| {
                    parent.spawn(BoxBundle {
                        component: Box,
                        position: GridCoords::new(corner.0 as i32, corner.1 as i32),
                        grid_entity: GridEntityInfo {
                            variant: "Box",
                            id: *id,
                            ..Default::default()
                        },
                        sprite_bundle: SpriteSheetBundle { 
                            sprite: TextureAtlasSprite {
                                index: 49,
                                ..Default::default()
                            },
                            texture_atlas: text_atlases.add(TextureAtlas::from_grid(
                                asset_server.load("tileset_alt.png").into(), 
                                Vec2::new(256.0, 256.0), 
                                8, 
                                8, 
                                None, 
                                None
                            )),
                            transform: Transform::from_xyz(
                                (TILE_SIZE * corner.0 + TILE_SIZE / 2) as f32,
                                (TILE_SIZE * corner.1 + TILE_SIZE / 2) as f32, 
                                0.0
                            ),
                            ..Default::default() 
                        },
                    });
                });
            },
            GridEntity::TimeMachine { id, grid, .. } => {
                for i in 0..grid.len() {
                    for j in 0..grid[i].len() {
                        commands.entity(bevy_level_entity).with_children(|parent| {
                            parent.spawn(TimeMachinePartBundle {
                                component: TimeMachine,
                                position: GridCoords::new((corner.0 + i)as i32, (corner.1 + j) as i32),
                                part_type: grid[i][j].0,
                                grid_entity: GridEntityInfo {
                                    variant: "TimeMachine",
                                    id: *id,
                                    pos: (i, j),
                                    ..Default::default()
                                },
                                sprite_bundle: SpriteSheetBundle { 
                                    sprite: TextureAtlasSprite {
                                        index: 36,
                                        ..Default::default()
                                    },
                                    texture_atlas: text_atlases.add(TextureAtlas::from_grid(
                                        asset_server.load("tileset_alt.png").into(), 
                                        Vec2::new(256.0, 256.0), 
                                        8, 
                                        8, 
                                        None, 
                                        None
                                    )),
                                    transform: Transform::from_xyz(
                                        (TILE_SIZE * (corner.0 + i) + TILE_SIZE / 2) as f32,
                                        (TILE_SIZE * (corner.1 + j) + TILE_SIZE / 2) as f32, 
                                        0.0
                                    ),
                                    ..Default::default() 
                                },
                            });
                        });
                    }
                }
            },
            _ => panic!("Shouldn't be trying to spawn a player or none")
        }
    }
}

#[derive(Resource, Debug, PartialEq, Clone)]
pub struct Grid {
    entities: Vec<((usize, usize), GridEntity)>,
    entity_grid: Vec<Vec<usize>>
}

impl Grid {
    pub fn new() -> Self {
        Self {
            entities: vec![((0, 0), GridEntity::None)],
            entity_grid: Vec::new()
        }
    }

    pub fn new_sized(width: usize, height: usize) -> Self {
        Self {
            entities: vec![((0, 0), GridEntity::None)],
            entity_grid: vec![vec![0; height]; width]
        }
    }

    pub fn height(&self) -> usize {
        self.entity_grid[0].len()
    }

    pub fn width(&self) -> usize {
        self.entity_grid.len()
    }

    pub fn num_entities(&self) -> usize {
        self.entities.len()
    }

    pub fn tm_contains_index(&self, time_machine: &GridEntity, search_index: usize) -> bool {
        time_machine.contains_index(search_index, &self.entities)
    }

    pub fn get_entity<'a>(&'a self, grid_entity_info: &GridEntityInfo) -> Option<&'a ((usize, usize), GridEntity)> {
        self.entities.iter().find(|(_, grid_entity)| {
            match (grid_entity, grid_entity_info.variant) {
                (GridEntity::Player { .. }, "Player") => true,
                (GridEntity::PastPlayer { id: entity_id, .. }, "PastPlayer") |
                (GridEntity::Box {id: entity_id, .. }, "Box") |
                (GridEntity::TimeMachine { id: entity_id, .. }, "TimeMachine") => *entity_id == grid_entity_info.id,
                _ => false
            }
        })
    }

    pub fn get_entity_mut<'a>(&'a mut self, grid_entity_info: &GridEntityInfo) -> Option<&'a mut ((usize, usize), GridEntity)> {
        self.entities.iter_mut().find(|(_, grid_entity)| {
            match (grid_entity, grid_entity_info.variant) {
                (GridEntity::Player { .. }, "Player") => true,
                (GridEntity::PastPlayer { id: entity_id, .. }, "PastPlayer") |
                (GridEntity::Box {id: entity_id, .. }, "Box") |
                (GridEntity::TimeMachine { id: entity_id, .. }, "TimeMachine") => *entity_id == grid_entity_info.id,
                _ => false
            }
        })
    }
    
    pub fn get_entity_from_id<'a>(&'a self, variant: &str, id: usize) -> Option<&'a ((usize, usize), GridEntity)> {
        self.entities.iter().find(|(_, grid_entity)| {
            match (grid_entity, variant) {
                (GridEntity::Player { .. }, "Player") => true,
                (GridEntity::PastPlayer { id: entity_id, .. }, "PastPlayer") |
                (GridEntity::Box {id: entity_id, .. }, "Box") |
                (GridEntity::TimeMachine { id: entity_id, .. }, "TimeMachine") => *entity_id == id,
                _ => false
            }
        })
    }

    pub fn get_entity_from_id_mut<'a>(&'a mut self, variant: &str, id: usize) -> Option<&'a mut ((usize, usize), GridEntity)> {
        self.entities.iter_mut().find(|(_, grid_entity)| {
            match (grid_entity, variant) {
                (GridEntity::Player { .. }, "Player") => true,
                (GridEntity::PastPlayer { id: entity_id, .. }, "PastPlayer") |
                (GridEntity::Box {id: entity_id, .. }, "Box") |
                (GridEntity::TimeMachine { id: entity_id, .. }, "TimeMachine") => *entity_id == id,
                _ => false
            }
        })
    }
    
    pub fn get_entity_from_coords<'a>(&'a self, coords: &GridCoords) -> Option<&'a ((usize, usize), GridEntity)> {
        if 0 <= coords.x && coords.x < self.width() as i32 && 0 <= coords.y && coords.y < self.height() as i32 {
            Some(&self.entities[self.entity_grid[coords.x as usize][coords.y as usize]])
        } else {
            None
        }
    }

    pub fn get_entity_from_coords_mut<'a>(&'a mut self, coords: &GridCoords) -> Option<&'a mut ((usize, usize), GridEntity)> {
        if 0 <= coords.x && coords.x < self.width() as i32 && 0 <= coords.y && coords.y < self.height() as i32 {
            Some(&mut self.entities[self.entity_grid[coords.x as usize][coords.y as usize]])
        } else {
            None
        }
    }
    
    pub fn get_entity_from_pos<'a>(&'a self, x: usize, y: usize) -> Option<&'a ((usize, usize), GridEntity)> {
        if x < self.width() && y < self.height() {
            Some(&self.entities[self.entity_grid[x][y]])
        } else {
            None
        }
    }

    pub fn get_entity_from_pos_mut<'a>(&'a mut self, x: usize, y: usize) -> Option<&'a mut ((usize, usize), GridEntity)> {
        if x < self.width() && y < self.height() {
            Some(&mut self.entities[self.entity_grid[x][y]])
        } else {
            None
        }
    }

    pub fn get_entity_index(&self, grid_entity_info: &GridEntityInfo) -> Option<usize> {
        self.entities.iter().position(|(_, grid_entity)| {
            match (grid_entity, grid_entity_info.variant) {
                (GridEntity::Player { .. }, "Player") => true,
                (GridEntity::PastPlayer { id: entity_id, .. }, "PastPlayer") |
                (GridEntity::Box {id: entity_id, .. }, "Box") |
                (GridEntity::TimeMachine { id: entity_id, .. }, "TimeMachine") => *entity_id == grid_entity_info.id,
                _ => false
            }
        })
    }

    pub fn get_entity_index_from_id(&self, variant: &str, id: usize) -> Option<usize> {
        self.entities.iter().position(|(_, grid_entity)| {
            match (grid_entity, variant) {
                (GridEntity::Player { .. }, "Player") => true,
                (GridEntity::PastPlayer { id: entity_id, .. }, "PastPlayer") |
                (GridEntity::Box {id: entity_id, .. }, "Box") |
                (GridEntity::TimeMachine { id: entity_id, .. }, "TimeMachine") => *entity_id == id,
                _ => false
            }
        })
    }

    pub fn get_all_of_type(&self, variant: &str) -> Vec<&((usize, usize), GridEntity)> {
        self.entities.iter().filter(|(_, grid_entity)| {
            match (grid_entity, variant) {
                (GridEntity::Player { .. }, "Player") => true,
                (GridEntity::PastPlayer { .. }, "PastPlayer") |
                (GridEntity::Box { .. }, "Box") |
                (GridEntity::TimeMachine { .. }, "TimeMachine") => true,
                _ => false
            }
        }).collect()
    }

    pub fn get_all_of_type_mut(&mut self, variant: &str) -> Vec<&mut ((usize, usize), GridEntity)> {
        self.entities.iter_mut().filter(|(_, grid_entity)| {
            match (grid_entity, variant) {
                (GridEntity::Player { .. }, "Player") => true,
                (GridEntity::PastPlayer { .. }, "PastPlayer") |
                (GridEntity::Box { .. }, "Box") |
                (GridEntity::TimeMachine { .. }, "TimeMachine") => true,
                _ => false
            }
        }).collect()
    }

    pub fn add_entity(&mut self, x: usize, y: usize, grid_entity_info: &GridEntityInfo) -> usize {
        self.get_entity_index(grid_entity_info).unwrap_or_else(|| {
            let id = grid_entity_info.id;

            self.entities.push(((x, y), match grid_entity_info.variant {
                "Player" => GridEntity::Player { movements: Vec::new() },
                "PastPlayer" => GridEntity::PastPlayer { id, movements: Vec::new() },
                "Box" => GridEntity::Box { id },
                "TimeMachine" => GridEntity::TimeMachine { 
                    start_instance: None, 
                    id, 
                    grid: Vec::new()
                },
                invalid_varient => panic!("Invalid GridEntity varient: {}", invalid_varient)
            }));
            
            self.entities.len() - 1
        })
    }

    // If varient and id exist in self.entities it just adds the index
    // If varient and id are valid, but don't exist it adds the entity to self.entities and the index
    // If variant is invalid it panics
    pub fn add_entity_to_pos(&mut self, x: usize, y: usize, grid_entity_info: &mut GridEntityInfo) {
        let index = self.add_entity(x, y, &*grid_entity_info);
        let current_index = &mut self.entity_grid[x][y];

        if *current_index == 0 {
            *current_index = index;
        } else if self.entities[*current_index].1.is_tm() {
            GridEntity::add_to_grid(
                *current_index, 
                index, 
                (x, y), 
                &mut self.entities
            );

            grid_entity_info.time_machine_depth = 1;
        } else if self.entities[index].1.is_tm() {
            GridEntity::add_to_grid(
                index, 
                *current_index, 
                self.entities[index].0, 
                &mut self.entities
            );

            *current_index = index;
        }
    }

    // !!Assumes that player is in the time machine!!
    pub fn replace_time_machine(&mut self, mut new_grid: Grid, time_machine_index: usize, t: usize, end_t: usize) {
        // println!("\n0.) {}\npast: {}","-".repeat(30) , self);
        
        let player_index = self.get_entity_index_from_id("Player", 0).unwrap();

        // replace player with past_player
        self.replace_player_to_pos(
            if let Some((_, GridEntity::Player { movements })) = &new_grid.get_entity_from_id("Player", 0) {
                movements[t..].to_vec()
            } else {
                panic!("Player index did index player (should be unreachable) index: {}, grid: {}", player_index, new_grid);
            }, 
            new_grid.get_entity_from_id("Player", 0).unwrap().0
        );

        // Remove the old player reference
        self.entities[ player_index ] = ((0, 0), GridEntity::None);

        // println!("\n1.) {}\npast: {}","-".repeat(30) , self);

        let (old_corner, ref old_tm_entity) = self.entities[ time_machine_index ];
        let old_contents = old_tm_entity.get_contents(&self.entities);
        
        let (new_corner, ref new_tm_entity) = new_grid.entities[ time_machine_index ];
        let new_contents = new_tm_entity.get_contents(&new_grid.entities);

        // println!("\n\nnew_contents: {:?}\nold_contents: {:?}\n\n", new_contents, old_contents);

        // Add the stuff that is new in the time machine and
        // Change the stuff that was cloned via the time machine
        for &index in new_contents.iter() {
            if self.entities.get( index ).is_none() {
                let new_index = self.num_entities();
                let pos = new_grid.entities[ index ].0;

                new_grid.change_all_indeces(index, new_index);
                self.entities.push(std::mem::take(&mut new_grid.entities[ index ]));
                // println!("{:?} - {:?} + {:?}", new_grid.entities[ new_index ].0, new_corner, old_corner);

                if pos.0 < new_corner.0 {
                    println!("index: {}\nnew_index: {}\nentity: {:?}\nnew_corner: {:?}", index, new_index, self.entities.last(), new_corner);
                }

                self.entities[ new_index ].0 = (
                    (pos.0 - new_corner.0) + old_corner.0,
                    (pos.1 - new_corner.1) + old_corner.1
                );
            } else if !old_contents.contains( &index ) {
                let new_index = self.num_entities();
                let mut new_entity = new_grid.entities[ index ].clone();
                new_entity.1.change_id(new_index);
                self.entities.push(new_entity.clone());
                new_grid.entities.push(new_entity);

                // println!("from_index: {}, to_index: {}, grid: {}", index, new_index, self);

                new_grid.change_all_indeces(index, new_index);
            }
        }

        // println!("\n2.) {}\npast: {}","-".repeat(30) , self);
        
        // Delete the stuff that was overided by the new time machine and
        // update the position of the stuff that stayed in the time machine
        for &index in old_contents.iter() {
            if new_contents.contains( &index ) { // Updating posititon
                // println!("new_grid_entity: {:?}, new_corner: {:?}, old_corner: {:?}", new_grid.entities[ index ].0, new_corner, old_corner);
                
                // println!("{:?} - {:?} + {:?}", new_grid.entities[ index ].0, new_corner, old_corner);
                self.entities[ index ].0 = (
                    (new_grid.entities[ index ].0.0 - new_corner.0) + old_corner.0,
                    (new_grid.entities[ index ].0.1 - new_corner.1) + old_corner.1
                );
            } else { // Deletion
                self.entities[ index ] = ((0, 0), GridEntity::None);
            }
        }

        // println!("\n3.) {}\npast: {}","-".repeat(30) , self);

        self.entities[ time_machine_index ].1 = new_grid.entities.remove(time_machine_index).1;

        // Sets the activation time for the time machine
        if let (_, GridEntity::TimeMachine { 
            start_instance: Some((_, Some(length), Some(end), _)), 
            .. 
        }) = &mut self.entities[ time_machine_index ] {
            *end += *length;

        } else if let (_, GridEntity::TimeMachine { 
            start_instance: Some((_, length, end, _)), 
            .. 
        }) = &mut self.entities[ time_machine_index ] {
            // *start = end_t;
            *length = Some(end_t - t);
            *end = Some(2 * end_t - t);
        }

        // println!("\n4.) {}\npast: {}","-".repeat(30) , self);
    }

    pub fn replace_player_to_pos(&mut self, movements: Vec<IVec2>, pos: (usize, usize)) {
        let player_index = self
            .get_entity_index_from_id("Player", 0)
            .expect("Could not find player in grid");

        // println!("movements: {:?}, player_index: {}", movements, player_index);

        let past_player = GridEntity::PastPlayer {
            movements: movements.clone(),
            id: self.entities.len()
        };

        self.entities.push((
            self.entities[ player_index ].0,
            past_player
        ));

        self.change_all_indeces(
            player_index, 
            self.entities.len() - 1
        );

        self.entities[ player_index ].0 = pos;
    }

    pub fn change_all_indeces(&mut self, from_index: usize, to_index: usize) {
        if from_index == to_index { return; }

        for i in 0..self.entity_grid.len() {
            for j in 0..self.entity_grid[i].len() {
                if self.entity_grid[ i ][ j ] == from_index {
                    self.entity_grid[ i ][ j ] = to_index
                } else if self.entity_grid[ i ][ j ] != to_index {
                    GridEntity::change_all_indeces(
                        self.entity_grid[ i ][ j ],
                        from_index, 
                        to_index, 
                        &mut self.entities
                    );
                }
            }
        }
    }

    pub fn update_events(&mut self, t: usize, clicked: &mut ClickedTimeMachine) {
        println!("t: {}", t);

        for i in 0..self.entities.len() {
            let stuff = match &mut self.entities[ i ].1 {
                GridEntity::PastPlayer { movements, .. } => {
                    (Some(movements.clone()), None)
                },
                GridEntity::TimeMachine { start_instance: Some((_, _, Some(departure_time), _)), .. } => {
                    (None, Some(*departure_time))
                },
                _ => {(None, None)}
            };

            if let (Some(mut movements), _) = stuff {
                // println!("past_player: {:?}", movements);

                if movements.len() == 0 { return; }

                let movement = movements.remove(0);

                if movement.to_array() != [0, 0] {
                    println!("{}", self.try_move(i, MoveDirection::from_ivec(movement)));
                }

                if let GridEntity::PastPlayer { movements: old_movements, .. } = &mut self.entities[ i ].1 { 
                    *old_movements = movements;
                    // println!("movements now: {:?}", old_movements);
                }

                // println!("{}", self);
            } else if let (_, Some(departure_time)) = stuff {
                println!("departure_time: {}", departure_time);
                if t >= departure_time {
                    clicked.0 = Some(GridEntityInfo::from(&self.entities[ i ].1))
                }
            }
        }
    }

    pub fn add_movement(&mut self, movement: IVec2) {
        // println!("add_movement (start): {}", self);

        for i in 0..self.entities.len() {
            let movements_opt = match &mut self.entities[ i ].1 {
                GridEntity::Player { movements, .. } => {
                    Some(movements)
                },
                GridEntity::PastPlayer { movements, .. } => {
                    Some(movements)
                },
                _ => {None}
            };

            if let Some(movements) = movements_opt {
                movements.push(movement);
            }
        }

        // println!("add_movement (end): {}", self);
    }

    pub fn try_move(&mut self, entity_index: usize, direction: MoveDirection) -> bool {
        let (x1, y1) = self.entities[entity_index].0;

        let (x2, y2) = match direction.get_changed_pos(&(x1, y1), self.width(), self.height()) {
            Some((x, y)) => (x, y),
            None => return false
        };

        let next_index = self.entity_index_at(x2, y2);

        if self.entity_at(x2, y2) == &GridEntity::None || (
            self.entity_at(x2, y2).is_pushable() &&
            self.try_move(next_index, direction)
        ) {
            self.set_to_pos(x1, y1, 0);
            self.set_to_pos(x2, y2, entity_index);
            self.entities[entity_index].0 = (x2, y2);

            true
        } else {
            false
        }
    }

    pub fn try_move_entity(&mut self, grid_entity_info: &GridEntityInfo, direction: MoveDirection) -> bool {
        if let Some(index) = self.get_entity_index(grid_entity_info) {
            self.try_move(index, direction)
        } else {
            false
        }
    }

    fn entity_at<'a>(&'a self, x: usize, y: usize) -> &'a GridEntity {
        let entity = &self.entities[self.entity_grid[ x ][ y ]];

        entity.1.entity_at(entity.0, (x, y), &self.entities)
    }

    fn entity_index_at(&mut self, x: usize, y: usize) -> usize {
        let entity = &self.entities[ self.entity_grid[ x ][ y ] ];

        if entity.1.is_tm() {
            entity.1.entity_index_at(
                entity.0, 
                (x, y), 
                &self.entities
            )
        } else {
            self.entity_grid[ x ][ y ]
        }
    }

    fn set_to_pos(&mut self, x: usize, y: usize, entity_index: usize) {
        let entity = &self.entities[ self.entity_grid[ x ][ y ] ];

        if entity.1.is_tm() {
            GridEntity::set_to_pos(
                self.entity_grid[ x ][ y ], 
                entity.0, 
                (x, y), 
                entity_index, 
                &mut self.entities
            );
        } else {
            self.entity_grid[ x ][ y ] = entity_index;
        }
    }

    pub fn set_depth_of(&self, grid_entity_info: &mut GridEntityInfo) {
        let index = self.get_entity_index(grid_entity_info).unwrap();
        let ((x, y), _) = self.entities[ index ];
        let entity = self.get_entity_from_pos(x, y).unwrap();

        grid_entity_info.time_machine_depth = entity.1.get_depth_of(
            entity.0,
            (x, y),
            index,
            &self.entities
        );
    }

    // Assumes that you will remove the places where the indeces are
    pub fn remove_entity(&mut self, grid_entity_info: &GridEntityInfo) {
        let index = self.get_entity_index(grid_entity_info).unwrap();
        self.entities[ index ] = ((0, 0), GridEntity::None);
    }

    pub fn remove_contents_of_entity(&mut self, grid_entity_info: &GridEntityInfo) {
        let contents = self.get_entity(grid_entity_info).unwrap().1.get_contents(&self.entities);

        for index in contents {
            self.entities[index] = ((0, 0), GridEntity::None);
        }
    }

    pub fn entities_iter(&self) -> core::slice::Iter<((usize, usize), GridEntity)> {
        self.entities.iter()
    }
}

impl std::fmt::Display for GridEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Player { movements } => {
                if true || movements.len() == 0 {
                    write!(f, "Player: [..]")
                } else {
                    write!(f, "Player: [{}\n        ]", 
                        movements.iter().fold(String::new(), |string, movement| format!("{}\n            {},", string, movement))
                    )
                }
            }, Self::PastPlayer { movements, id } => {
                if true || movements.len() == 0 {
                    write!(f, "PastPlayer[{}]: [..]", id)
                } else {
                    write!(f, "PastPlayer[{}]: [{}\n        ]", id,
                        movements.iter().fold(String::new(), |string, movement| format!("{}\n            {},", string, movement))
                    )
                }
            }, Self::Box { id } => {
                write!(f, "Box[ {} ]", id)
            }, Self::TimeMachine { id, grid, .. } => {
                write!(f, "TimeMachine[ {} ]: [{}\n        ]", id,
                    grid.iter().fold(String::new(), |string, row| {format!("{}\n            [{}  ],", string, 
                        row.iter().fold(String::new(), |string, (_part, num)| {format!("{} {:2}", string, num)})
                    )})
                )
            }, Self::None => {
                write!(f, "None")
            }
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\
            {{\n    \
                Entities: [{}\n    ],\n    \
                Grid: [{}\n    ]\n\
            }}", 
            self.entities.iter().enumerate().fold(String::new(), |string, (i, (pos, entity))| format!("{}\n        [{}] @ {:?} -> {},", string, i, pos, entity)), 
            self.entity_grid.iter().fold(String::new(), |string, row| {
                format!("{}\n        [{}  ],", string, row.iter().fold(String::new(), |string, num| format!("{} {:2}", string, num)))
            })
        )
    }
}