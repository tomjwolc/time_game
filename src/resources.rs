use super::*;

#[derive(Resource)]
pub struct Dims {
    pub x: usize,
    pub y: usize
}

#[derive(Debug)]
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

#[derive(Resource, Debug)]
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

    pub fn get_entity<'a>(&'a self, variant: &str, id: usize) -> Option<&'a ((usize, usize), GridEntity)> {
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

    pub fn get_entity_mut<'a>(&'a mut self, variant: &str, id: usize) -> Option<&'a mut ((usize, usize), GridEntity)> {
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
    
    pub fn get_entity_from_coords<'a>(&'a self, coords: GridCoords) -> Option<&'a ((usize, usize), GridEntity)> {
        
    }

    fn get_entity_index(&self, variant: &str, id: usize) -> Option<usize> {
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

    pub fn add_entity(&mut self, x: usize, y: usize, variant: &str, id: usize) -> usize {
        self.get_entity_index(variant, id).unwrap_or_else(|| {
            self.entities.push(((x, y), match variant {
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
    pub fn add_entity_to_pos(&mut self, x: usize, y: usize, variant: &str, id: usize) {
        let index = self.get_entity_index(variant, id).unwrap_or_else(|| {
            self.entities.push(((x, y), match variant {
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
        });

        self.entity_grid[x][y] = index;
    }
}