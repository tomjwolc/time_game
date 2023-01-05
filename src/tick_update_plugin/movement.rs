use super::*;

#[derive(Clone, Copy)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right
}

impl MoveDirection {
    pub fn chnage_coords(&self, coords: &mut GridCoords) {
        match self {
            Self::Up => coords.y += 1,
            Self::Down => coords.y -= 1,
            Self::Left => coords.x -= 1,
            Self::Right => coords.x += 1
        }
    }

    pub fn get_changed_pos(&self, pos: &(usize, usize), max_x: usize, max_y: usize) -> Option<(usize, usize)> {
        match self {
            Self::Up    if pos.1 < max_y - 1 => Some((   pos.0   , pos.1 + 1 )),
            Self::Down  if pos.1 > 0         => Some((   pos.0   , pos.1 - 1 )),
            Self::Left  if pos.0 > 0         => Some(( pos.0 - 1 ,   pos.1   )),
            Self::Right if pos.0 < max_x - 1 => Some(( pos.0 + 1 ,   pos.1   )),
            _                                => None
        }
    } 

    pub fn from_ivec(ivec: IVec2) -> Self {
        match ivec.to_array() {
            [0, 1] => MoveDirection::Up,
            [0, -1] => MoveDirection::Down,
            [-1, 0] => MoveDirection::Left,
            [1, 0] => MoveDirection::Right,
            vec => panic!("Cannot convert {:?} to MoveDirection", vec)
        }
    }
}

pub fn movement_event_happened(key_event: Res<KeyEvent>) -> bool {
    key_event.to_direction().is_some()
}

pub fn player_movement(
    key_event: Res<KeyEvent>,
    mut query: Query<&GridEntityInfo, With<Player>>,
    mut grid: ResMut<Grid>
) {
    if let Ok(grid_entity_info) = query.get_single_mut() {
        let direction = key_event.to_direction().unwrap();
        
        if grid.try_move_entity(grid_entity_info, direction) {
            println!("grid:{}", *grid);
        }
    }
}