use super::*;

#[derive(Debug, PartialEq)]
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
        start_instance: Option<Grid>,
        id: usize, 
        grid: Vec<Vec<(TimeMachinePartType, usize)>>
    },
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
}

#[derive(Resource, Debug, PartialEq)]
pub struct Grid {
    ghost_player_num: usize,
    entities: Vec<((usize, usize), GridEntity)>,
    entity_grid: Vec<Vec<usize>>
}

impl Grid {
    pub fn new() -> Self {
        Self {
            ghost_player_num: 0,
            entities: vec![((0, 0), GridEntity::None)],
            entity_grid: Vec::new()
        }
    }

    pub fn new_sized(width: usize, height: usize) -> Self {
        Self {
            ghost_player_num: 0,
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

    fn get_entity_index(&self, grid_entity_info: &GridEntityInfo) -> Option<usize> {
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
    pub fn add_entity_to_pos(&mut self, x: usize, y: usize, grid_entity_info: &GridEntityInfo) {
        let index = self.add_entity(x, y, grid_entity_info);
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
}

impl std::fmt::Display for GridEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Player { movements } => {
                if movements.len() == 0 {
                    write!(f, "Player: []")
                } else {
                    write!(f, "Player: [{}\n        ]", 
                        movements.iter().fold(String::new(), |string, movement| format!("{}\n            {},", string, movement))
                    )
                }
            }, Self::PastPlayer { movements, id } => {
                if movements.len() == 0 {
                    write!(f, "PastPlayer[{}]: []", id)
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
                Number of Ghost Players: {}\n    \
                Entities: [{}\n    ],\n    \
                Grid: [{}\n    ]\n\
            }}", 
            self.ghost_player_num,
            self.entities.iter().enumerate().fold(String::new(), |string, (i, (pos, entity))| format!("{}\n        [{}] @ {:?} -> {},", string, i, pos, entity)), 
            self.entity_grid.iter().fold(String::new(), |string, row| {
                format!("{}\n        [{}  ],", string, row.iter().fold(String::new(), |string, num| format!("{} {:2}", string, num)))
            })
        )
    }
}