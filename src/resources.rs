use super::*;

pub struct EntityState<'a> {
    coords: &'a mut GridCoords,
    transform: &'a mut Transform
}

#[derive(Resource)]
pub struct MasterTimeline<'a>(pub Vec<Vec<EntityState<'a>>>);

#[derive(Resource)]
pub struct T(pub usize);

#[derive(Clone, Copy, Debug)]
pub enum GridState {
    Nothing,
    GhostPlayer,
    Player,
    Box,
    TimeMachine(usize, TimeMachinePartType)
}

impl GridState {
    pub fn to_tm(&self) -> Option<(&usize, &TimeMachinePartType)> {
        if let GridState::TimeMachine(id, part_type) = self {
            Some((id, part_type))
        } else {
            None
        }
    }

    pub fn is_tm(&self) -> bool {
        if let GridState::TimeMachine(_, _) = self {
            true
        } else {
            false
        }
    }
}

#[derive(Resource)]
pub struct EntityGrid(pub Vec<Vec<GridState>>);