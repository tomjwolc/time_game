// use crate::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub mod update_entities;
pub use update_entities::*;

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Component)]
pub struct PastPlayer;

#[derive(Default, Component)]
pub struct Box;

#[derive(Default, Component)]
pub struct TimeMachine;

#[derive(Default, Component, Debug)]
pub struct GridEntityInfo {
    pub variant: &'static str,
    pub id: usize,
    pub pos: (usize, usize)
}

impl GridEntityInfo {
    fn player(_: EntityInstance) -> Self {
        Self {
            variant: "Player",
            id: 0,
            pos: (0, 0)
        }
    }

    fn past_player(_: EntityInstance) -> Self {
        Self {
            variant: "PastPlayer",
            id: 0, // to be changed later
            pos: (0, 0)
        }
    }

    fn box_entity(_: EntityInstance) -> Self {
        Self {
            variant: "Box",
            id: 0, // to be changed later
            pos: (0, 0)
        }
    }

    fn time_machine(_: EntityInstance) -> Self {
        Self {
            variant: "TimeMachine",
            id: 0, // to be changed later
            pos: (0, 0) // to be changed later
        }
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    component: Player,
    #[with(GridEntityInfo::player)]
    grid_entity: GridEntityInfo,
    #[grid_coords]
    position: GridCoords,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle
}

#[derive(Bundle, LdtkEntity)]
pub struct PastPlayerBundle {
    component: PastPlayer,
    #[with(GridEntityInfo::past_player)]
    grid_entity: GridEntityInfo,
    #[grid_coords]
    position: GridCoords,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle
}

#[derive(Bundle, LdtkEntity)]
pub struct BoxBundle {
    component: Box,
    #[with(GridEntityInfo::box_entity)]
    grid_entity: GridEntityInfo,
    #[grid_coords]
    position: GridCoords,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle
}

pub fn update_transform(coords: &GridCoords, transform: &mut Transform) {
    transform.translation.x = coords.x as f32 * 256.0 + 128.0;
    transform.translation.y = coords.y as f32 * 256.0 + 128.0;
}

#[derive(Default, Component, Debug, Clone, Copy, PartialEq)]
pub enum TimeMachinePartType {
    TopLeftFull,
    TopRightFull,
    BottomLeftFull,
    BottomRightFull,

    #[default]
    Middle,

    TopFull,
    BottomFull,
    LeftFull,
    RightFull,

    LeftTangentTop,
    RightTangentTop,
    LeftTangentBottom,
    RightTangentBottom,
    TopTangentLeft,
    BottomTangentLeft,
    TopTangentRight,
    BottomTangentRight,

    LeftPerpTop,
    RightPerpTop,
    LeftPerpBottom,
    RightPerpBottom,
    TopPerpLeft,
    BottomPerpLeft,
    TopPerpRight,
    BottomPerpRight,

    TopOpening,
    BottomOpening,
    LeftOpening,
    RightOpening,

    TopLeftTangentRight,
    BottomLeftTangentRight,
    TopRightTangentLeft,
    BottomRightTangentLeft,
    TopLeftTangentBottom,
    BottomLeftTangentTop,
    TopRightTangentBottom,
    BottomRightTangentTop,

    MiddleTopOpen,
    MiddleBottomOpen,
    MiddleLeftOpen,
    MiddleRightOpen
}

impl TimeMachinePartType {
    pub fn to_num(&self) -> usize {
        match self {TimeMachinePartType::TopLeftFull => 0,
            TimeMachinePartType::TopRightFull => 1,
            TimeMachinePartType::BottomLeftFull => 2,
            TimeMachinePartType::BottomRightFull => 3,
            TimeMachinePartType::Middle => 4,
            TimeMachinePartType::TopFull => 5,
            TimeMachinePartType::BottomFull => 6,
            TimeMachinePartType::LeftFull => 7,
            TimeMachinePartType::RightFull => 8,
            TimeMachinePartType::LeftTangentTop => 9,
            TimeMachinePartType::RightTangentTop => 10,
            TimeMachinePartType::LeftTangentBottom => 11,
            TimeMachinePartType::RightTangentBottom => 12,
            TimeMachinePartType::TopTangentLeft => 13,
            TimeMachinePartType::BottomTangentLeft => 14,
            TimeMachinePartType::TopTangentRight => 15,
            TimeMachinePartType::BottomTangentRight => 16,
            TimeMachinePartType::LeftPerpTop => 17,
            TimeMachinePartType::RightPerpTop => 18,
            TimeMachinePartType::LeftPerpBottom => 19,
            TimeMachinePartType::RightPerpBottom => 20,
            TimeMachinePartType::TopPerpLeft => 21,
            TimeMachinePartType::BottomPerpLeft => 22,
            TimeMachinePartType::TopPerpRight => 23,
            TimeMachinePartType::BottomPerpRight => 24,
            TimeMachinePartType::TopOpening => 25,
            TimeMachinePartType::BottomOpening => 26,
            TimeMachinePartType::LeftOpening => 27,
            TimeMachinePartType::RightOpening => 28,
            TimeMachinePartType::TopLeftTangentRight => 29,
            TimeMachinePartType::BottomLeftTangentRight => 30,
            TimeMachinePartType::TopRightTangentLeft => 31,
            TimeMachinePartType::BottomRightTangentLeft => 32,
            TimeMachinePartType::TopLeftTangentBottom => 33,
            TimeMachinePartType::BottomLeftTangentTop => 34,
            TimeMachinePartType::TopRightTangentBottom => 35,
            TimeMachinePartType::BottomRightTangentTop => 36,
            TimeMachinePartType::MiddleTopOpen => 37,
            TimeMachinePartType::MiddleBottomOpen => 38,
            TimeMachinePartType::MiddleLeftOpen => 39,
            TimeMachinePartType::MiddleRightOpen => 40,
        }
    }

    pub fn fits_on_top(&self, next: &Self) -> bool {
        match (self, next) {
            // Straight up cannot have anything connecting to the top
            (part_type, _) if [
                TimeMachinePartType::TopLeftFull,
                TimeMachinePartType::TopRightFull,
                TimeMachinePartType::TopFull,

                TimeMachinePartType::TopTangentLeft,
                TimeMachinePartType::TopTangentRight,

                TimeMachinePartType::LeftPerpTop,
                TimeMachinePartType::RightPerpTop,
                TimeMachinePartType::TopPerpLeft,
                TimeMachinePartType::TopPerpRight,

                TimeMachinePartType::TopOpening,
                TimeMachinePartType::BottomOpening,
                TimeMachinePartType::LeftOpening,
                TimeMachinePartType::RightOpening,

                TimeMachinePartType::TopLeftTangentRight,
                TimeMachinePartType::TopRightTangentLeft,
                TimeMachinePartType::TopLeftTangentBottom,
                TimeMachinePartType::TopRightTangentBottom,
                TimeMachinePartType::MiddleTopOpen,
            ].contains(part_type) => false,

            // Left line connection
            (part_type1, part_type2) if [
                TimeMachinePartType::BottomLeftFull,
                TimeMachinePartType::LeftFull,

                TimeMachinePartType::LeftTangentBottom,
                TimeMachinePartType::LeftPerpBottom,
                
                TimeMachinePartType::BottomLeftTangentRight,
            ].contains(part_type1) && [
                TimeMachinePartType::TopLeftFull,
                TimeMachinePartType::LeftFull,

                TimeMachinePartType::LeftTangentTop,
                TimeMachinePartType::LeftPerpTop,
                
                TimeMachinePartType::TopLeftTangentRight,
            ].contains(part_type2) => true,

            // Left edge connection
            (part_type1, part_type2) if [
                TimeMachinePartType::MiddleLeftOpen,
                TimeMachinePartType::BottomLeftTangentTop,
            ].contains(part_type1) && [
                TimeMachinePartType::MiddleLeftOpen,
                TimeMachinePartType::TopLeftTangentBottom,
            ].contains(part_type2) => true,

            // Right line connection
            (part_type1, part_type2) if [
                TimeMachinePartType::BottomRightFull,
                TimeMachinePartType::RightFull,

                TimeMachinePartType::RightTangentBottom,
                TimeMachinePartType::RightPerpBottom,
                
                TimeMachinePartType::BottomRightTangentLeft,
            ].contains(part_type1) && [
                TimeMachinePartType::TopRightFull,
                TimeMachinePartType::RightFull,

                TimeMachinePartType::RightTangentTop,
                TimeMachinePartType::RightPerpTop,
                
                TimeMachinePartType::TopRightTangentLeft,
            ].contains(part_type2) => true,
            
            // Right edge connection
            (part_type1, part_type2) if [
                TimeMachinePartType::MiddleRightOpen,
                TimeMachinePartType::BottomRightTangentTop,
            ].contains(part_type1) && [
                TimeMachinePartType::MiddleRightOpen,
                TimeMachinePartType::TopRightTangentBottom,
            ].contains(part_type2) => true,

            // Whole connection
            (part_type1, part_type2) if [
                TimeMachinePartType::Middle,
                TimeMachinePartType::BottomFull,

                TimeMachinePartType::BottomTangentLeft,
                TimeMachinePartType::BottomTangentRight,

                TimeMachinePartType::MiddleBottomOpen,
            ].contains(part_type1) && [
                TimeMachinePartType::Middle,
                TimeMachinePartType::TopFull,

                TimeMachinePartType::TopTangentLeft,
                TimeMachinePartType::TopTangentRight,

                TimeMachinePartType::MiddleTopOpen,
            ].contains(part_type2) => true,
            (_, _) => false
        }
    }
    
    pub fn fits_on_bottom(&self, next: &Self) -> bool {
        match (self, next) {
            // Straight up cannot have anything connecting to the top
            (part_type, _) if [
                TimeMachinePartType::BottomLeftFull,
                TimeMachinePartType::BottomRightFull,
                TimeMachinePartType::TopFull,

                TimeMachinePartType::BottomTangentLeft,
                TimeMachinePartType::BottomTangentRight,

                TimeMachinePartType::LeftPerpBottom,
                TimeMachinePartType::RightPerpBottom,
                TimeMachinePartType::BottomPerpLeft,
                TimeMachinePartType::BottomPerpRight,

                TimeMachinePartType::TopOpening,
                TimeMachinePartType::BottomOpening,
                TimeMachinePartType::LeftOpening,
                TimeMachinePartType::RightOpening,

                TimeMachinePartType::BottomLeftTangentRight,
                TimeMachinePartType::BottomRightTangentLeft,
                TimeMachinePartType::BottomLeftTangentTop,
                TimeMachinePartType::BottomRightTangentTop,
                TimeMachinePartType::MiddleBottomOpen,
            ].contains(part_type) => false,

            // Left line connection
            (part_type1, part_type2) if [
                TimeMachinePartType::TopLeftFull,
                TimeMachinePartType::LeftFull,

                TimeMachinePartType::LeftTangentTop,
                TimeMachinePartType::LeftPerpTop,
                
                TimeMachinePartType::TopLeftTangentRight,
            ].contains(part_type1) && [
                TimeMachinePartType::BottomLeftFull,
                TimeMachinePartType::LeftFull,

                TimeMachinePartType::LeftTangentBottom,
                TimeMachinePartType::LeftPerpBottom,
                
                TimeMachinePartType::BottomLeftTangentRight,
            ].contains(part_type2) => true,

            // Left edge connection
            (part_type1, part_type2) if [
                TimeMachinePartType::MiddleLeftOpen,
                TimeMachinePartType::TopLeftTangentBottom,
            ].contains(part_type1) && [
                TimeMachinePartType::MiddleLeftOpen,
                TimeMachinePartType::BottomLeftTangentTop,
            ].contains(part_type2) => true,

            // Right line connection
            (part_type1, part_type2) if [
                TimeMachinePartType::TopRightFull,
                TimeMachinePartType::RightFull,

                TimeMachinePartType::RightTangentTop,
                TimeMachinePartType::RightPerpTop,
                
                TimeMachinePartType::TopRightTangentLeft,
            ].contains(part_type1) && [
                TimeMachinePartType::BottomRightFull,
                TimeMachinePartType::RightFull,

                TimeMachinePartType::RightTangentBottom,
                TimeMachinePartType::RightPerpBottom,
                
                TimeMachinePartType::BottomRightTangentLeft,
            ].contains(part_type2) => true,
            
            // Right edge connection
            (part_type1, part_type2) if [
                TimeMachinePartType::MiddleRightOpen,
                TimeMachinePartType::TopRightTangentBottom,
            ].contains(part_type1) && [
                TimeMachinePartType::MiddleRightOpen,
                TimeMachinePartType::BottomRightTangentTop,
            ].contains(part_type2) => true,

            // Whole connection
            (part_type1, part_type2) if [
                TimeMachinePartType::Middle,
                TimeMachinePartType::TopFull,

                TimeMachinePartType::TopTangentLeft,
                TimeMachinePartType::TopTangentRight,

                TimeMachinePartType::MiddleTopOpen,
            ].contains(part_type1) && [
                TimeMachinePartType::Middle,
                TimeMachinePartType::BottomFull,

                TimeMachinePartType::BottomTangentLeft,
                TimeMachinePartType::BottomTangentRight,

                TimeMachinePartType::MiddleBottomOpen,
            ].contains(part_type2) => true,
            (_, _) => false
        }
    }

    pub fn fits_on_left(&self, next: &Self) -> bool {
        match (self, next) {
            // Straight up cannot have anything connecting to the top
            (part_type, _) if [
                TimeMachinePartType::BottomLeftFull,
                TimeMachinePartType::TopLeftFull,
                TimeMachinePartType::LeftFull,

                TimeMachinePartType::LeftTangentTop,
                TimeMachinePartType::LeftTangentBottom,

                TimeMachinePartType::TopPerpLeft,
                TimeMachinePartType::BottomPerpLeft,
                TimeMachinePartType::LeftPerpTop,
                TimeMachinePartType::LeftPerpBottom,

                TimeMachinePartType::TopOpening,
                TimeMachinePartType::BottomOpening,
                TimeMachinePartType::LeftOpening,
                TimeMachinePartType::RightOpening,

                TimeMachinePartType::BottomLeftTangentRight,
                TimeMachinePartType::TopLeftTangentRight,
                TimeMachinePartType::BottomLeftTangentTop,
                TimeMachinePartType::TopLeftTangentBottom,
                TimeMachinePartType::MiddleLeftOpen,
            ].contains(part_type) => false,

            // Top line connection
            (part_type1, part_type2) if [
                TimeMachinePartType::TopRightFull,
                TimeMachinePartType::TopFull,

                TimeMachinePartType::TopTangentRight,
                TimeMachinePartType::TopPerpRight,
                
                TimeMachinePartType::TopRightTangentBottom,
            ].contains(part_type1) && [
                TimeMachinePartType::TopLeftFull,
                TimeMachinePartType::TopFull,

                TimeMachinePartType::TopTangentLeft,
                TimeMachinePartType::TopPerpLeft,
                
                TimeMachinePartType::TopLeftTangentBottom,
            ].contains(part_type2) => true,

            // Top edge connection
            (part_type1, part_type2) if [
                TimeMachinePartType::MiddleTopOpen,
                TimeMachinePartType::TopRightTangentLeft,
            ].contains(part_type1) && [
                TimeMachinePartType::MiddleTopOpen,
                TimeMachinePartType::TopLeftTangentRight,
            ].contains(part_type2) => true,

            // Bottom line connection
            (part_type1, part_type2) if [
                TimeMachinePartType::BottomRightFull,
                TimeMachinePartType::BottomFull,

                TimeMachinePartType::BottomTangentRight,
                TimeMachinePartType::BottomPerpRight,
                
                TimeMachinePartType::BottomRightTangentTop,
            ].contains(part_type1) && [
                TimeMachinePartType::BottomLeftFull,
                TimeMachinePartType::BottomFull,

                TimeMachinePartType::BottomTangentLeft,
                TimeMachinePartType::BottomPerpLeft,
                
                TimeMachinePartType::BottomLeftTangentTop,
            ].contains(part_type2) => true,
            
            // Bottom edge connection
            (part_type1, part_type2) if [
                TimeMachinePartType::MiddleBottomOpen,
                TimeMachinePartType::BottomRightTangentLeft,
            ].contains(part_type1) && [
                TimeMachinePartType::MiddleBottomOpen,
                TimeMachinePartType::BottomLeftTangentRight,
            ].contains(part_type2) => true,

            // Whole connection
            (part_type1, part_type2) if [
                TimeMachinePartType::Middle,
                TimeMachinePartType::RightFull,

                TimeMachinePartType::RightTangentTop,
                TimeMachinePartType::RightTangentBottom,

                TimeMachinePartType::MiddleRightOpen,
            ].contains(part_type1) && [
                TimeMachinePartType::Middle,
                TimeMachinePartType::LeftFull,

                TimeMachinePartType::LeftTangentTop,
                TimeMachinePartType::LeftTangentBottom,

                TimeMachinePartType::MiddleLeftOpen,
            ].contains(part_type2) => true,
            (_, _) => false
        }
    }

    pub fn fits_on_right(&self, next: &Self) -> bool {
        match (self, next) {
            // Straight up cannot have anything connecting to the top
            (part_type, _) if [
                TimeMachinePartType::BottomRightFull,
                TimeMachinePartType::TopRightFull,
                TimeMachinePartType::RightFull,

                TimeMachinePartType::RightTangentTop,
                TimeMachinePartType::RightTangentBottom,

                TimeMachinePartType::TopPerpRight,
                TimeMachinePartType::BottomPerpRight,
                TimeMachinePartType::RightPerpTop,
                TimeMachinePartType::RightPerpBottom,

                TimeMachinePartType::TopOpening,
                TimeMachinePartType::BottomOpening,
                TimeMachinePartType::RightOpening,
                TimeMachinePartType::LeftOpening,

                TimeMachinePartType::BottomRightTangentLeft,
                TimeMachinePartType::TopRightTangentLeft,
                TimeMachinePartType::BottomRightTangentTop,
                TimeMachinePartType::TopRightTangentBottom,
                TimeMachinePartType::MiddleRightOpen,
            ].contains(part_type) => false,

            // Top line connection
            (part_type1, part_type2) if [
                TimeMachinePartType::TopLeftFull,
                TimeMachinePartType::TopFull,

                TimeMachinePartType::TopTangentLeft,
                TimeMachinePartType::TopPerpLeft,
                
                TimeMachinePartType::TopLeftTangentBottom,
            ].contains(part_type1) && [
                TimeMachinePartType::TopRightFull,
                TimeMachinePartType::TopFull,

                TimeMachinePartType::TopTangentRight,
                TimeMachinePartType::TopPerpRight,
                
                TimeMachinePartType::TopRightTangentBottom,
            ].contains(part_type2) => true,

            // Top edge connection
            (part_type1, part_type2) if [
                TimeMachinePartType::MiddleTopOpen,
                TimeMachinePartType::TopLeftTangentRight,
            ].contains(part_type1) && [
                TimeMachinePartType::MiddleTopOpen,
                TimeMachinePartType::TopRightTangentLeft,
            ].contains(part_type2) => true,

            // Bottom line connection
            (part_type1, part_type2) if [
                TimeMachinePartType::BottomLeftFull,
                TimeMachinePartType::BottomFull,

                TimeMachinePartType::BottomTangentLeft,
                TimeMachinePartType::BottomPerpLeft,
                
                TimeMachinePartType::BottomLeftTangentTop,
            ].contains(part_type1) && [
                TimeMachinePartType::BottomRightFull,
                TimeMachinePartType::BottomFull,

                TimeMachinePartType::BottomTangentRight,
                TimeMachinePartType::BottomPerpRight,
                
                TimeMachinePartType::BottomRightTangentTop,
            ].contains(part_type2) => true,
            
            // Bottom edge connection
            (part_type1, part_type2) if [
                TimeMachinePartType::MiddleBottomOpen,
                TimeMachinePartType::BottomLeftTangentRight,
            ].contains(part_type1) && [
                TimeMachinePartType::MiddleBottomOpen,
                TimeMachinePartType::BottomRightTangentLeft,
            ].contains(part_type2) => true,

            // Whole connection
            (part_type1, part_type2) if [
                TimeMachinePartType::Middle,
                TimeMachinePartType::LeftFull,

                TimeMachinePartType::LeftTangentTop,
                TimeMachinePartType::LeftTangentBottom,

                TimeMachinePartType::MiddleLeftOpen,
            ].contains(part_type1) && [
                TimeMachinePartType::Middle,
                TimeMachinePartType::RightFull,

                TimeMachinePartType::RightTangentTop,
                TimeMachinePartType::RightTangentBottom,

                TimeMachinePartType::MiddleRightOpen,
            ].contains(part_type2) => true,
            (_, _) => false
        }
    }


    pub fn can_enter_exit_top(&self) -> bool {
        match self {
            TimeMachinePartType::TopLeftFull |
            TimeMachinePartType::TopRightFull |
            TimeMachinePartType::TopFull |

            TimeMachinePartType::TopTangentLeft |
            TimeMachinePartType::TopTangentRight |
            TimeMachinePartType::TopPerpLeft |
            TimeMachinePartType::TopPerpRight |

            TimeMachinePartType::LeftOpening |
            TimeMachinePartType::BottomOpening |
            TimeMachinePartType::RightOpening |

            TimeMachinePartType::TopLeftTangentRight |
            TimeMachinePartType::TopLeftTangentBottom |
            TimeMachinePartType::TopRightTangentLeft |
            TimeMachinePartType::TopRightTangentBottom => false,
            _ => true
        }
    }

    pub fn can_enter_exit_bottom(&self) -> bool {
        match self {
            TimeMachinePartType::BottomLeftFull |
            TimeMachinePartType::BottomRightFull |
            TimeMachinePartType::BottomFull |

            TimeMachinePartType::BottomTangentLeft |
            TimeMachinePartType::BottomTangentRight |
            TimeMachinePartType::BottomPerpLeft |
            TimeMachinePartType::BottomPerpRight |

            TimeMachinePartType::LeftOpening |
            TimeMachinePartType::TopOpening |
            TimeMachinePartType::RightOpening |

            TimeMachinePartType::BottomLeftTangentRight |
            TimeMachinePartType::BottomLeftTangentTop |
            TimeMachinePartType::BottomRightTangentLeft |
            TimeMachinePartType::BottomRightTangentTop => false,
            _ => true
        }
    }

    pub fn can_enter_exit_left(&self) -> bool {
        match self {
            TimeMachinePartType::TopLeftFull |
            TimeMachinePartType::BottomLeftFull |
            TimeMachinePartType::LeftFull |

            TimeMachinePartType::LeftTangentTop |
            TimeMachinePartType::LeftTangentBottom |
            TimeMachinePartType::LeftPerpTop |
            TimeMachinePartType::LeftPerpBottom |

            TimeMachinePartType::TopOpening |
            TimeMachinePartType::BottomOpening |
            TimeMachinePartType::RightOpening |

            TimeMachinePartType::TopLeftTangentRight |
            TimeMachinePartType::TopLeftTangentBottom |
            TimeMachinePartType::BottomLeftTangentRight |
            TimeMachinePartType::BottomLeftTangentTop => false,
            _ => true
        }
    }

    pub fn can_enter_exit_right(&self) -> bool {
        match self {
            TimeMachinePartType::TopRightFull |
            TimeMachinePartType::BottomRightFull |
            TimeMachinePartType::RightFull |

            TimeMachinePartType::RightTangentTop |
            TimeMachinePartType::RightTangentBottom |
            TimeMachinePartType::RightPerpTop |
            TimeMachinePartType::RightPerpBottom |

            TimeMachinePartType::TopOpening |
            TimeMachinePartType::BottomOpening |
            TimeMachinePartType::LeftOpening |

            TimeMachinePartType::TopRightTangentLeft |
            TimeMachinePartType::TopRightTangentBottom |
            TimeMachinePartType::BottomRightTangentLeft |
            TimeMachinePartType::BottomRightTangentTop => false,
            _ => true
        }
    }

    pub fn get_type(entity_instance: EntityInstance) -> Self {
        entity_instance
            .field_instances[0]
            .real_editor_values[0]
            .as_ref().unwrap()
            .get("params").unwrap()
            .get(0).unwrap()
            .as_str().unwrap()
            .parse().unwrap()
    }
}

impl std::str::FromStr for TimeMachinePartType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TopLeftFull" => Ok(TimeMachinePartType::TopLeftFull),
            "TopRightFull" => Ok(TimeMachinePartType::TopRightFull),
            "BottomLeftFull" => Ok(TimeMachinePartType::BottomLeftFull),
            "BottomRightFull" => Ok(TimeMachinePartType::BottomRightFull),
            "Middle" => Ok(TimeMachinePartType::Middle),
            "TopFull" => Ok(TimeMachinePartType::TopFull),
            "BottomFull" => Ok(TimeMachinePartType::BottomFull),
            "LeftFull" => Ok(TimeMachinePartType::LeftFull),
            "RightFull" => Ok(TimeMachinePartType::RightFull),

            "LeftTangentTop" => Ok(TimeMachinePartType::LeftTangentTop),
            "RightTangentTop" => Ok(TimeMachinePartType::RightTangentTop),
            "LeftTangentBottom" => Ok(TimeMachinePartType::LeftTangentBottom),
            "RightTangentBottom" => Ok(TimeMachinePartType::RightTangentBottom),
            "TopTangentLeft" => Ok(TimeMachinePartType::TopTangentLeft),
            "BottomTangentLeft" => Ok(TimeMachinePartType::BottomTangentLeft),
            "TopTangentRight" => Ok(TimeMachinePartType::TopTangentRight),
            "BottomTangentRight" => Ok(TimeMachinePartType::BottomTangentRight),

            "LeftPerpTop" => Ok(TimeMachinePartType::LeftPerpTop),
            "RightPerpTop" => Ok(TimeMachinePartType::RightPerpTop),
            "LeftPerpBottom" => Ok(TimeMachinePartType::LeftPerpBottom),
            "RightPerpBottom" => Ok(TimeMachinePartType::RightPerpBottom),
            "TopPerpLeft" => Ok(TimeMachinePartType::TopPerpLeft),
            "BottomPerpLeft" => Ok(TimeMachinePartType::BottomPerpLeft),
            "TopPerpRight" => Ok(TimeMachinePartType::TopPerpRight),
            "BottomPerpRight" => Ok(TimeMachinePartType::BottomPerpRight),

            "TopOpening" => Ok(TimeMachinePartType::TopOpening),
            "BottomOpening" => Ok(TimeMachinePartType::BottomOpening),
            "LeftOpening" => Ok(TimeMachinePartType::LeftOpening),
            "RightOpening" => Ok(TimeMachinePartType::RightOpening),

            "TopLeftTangentRight" => Ok(TimeMachinePartType::TopLeftTangentRight),
            "BottomLeftTangentRight" => Ok(TimeMachinePartType::BottomLeftTangentRight),
            "TopRightTangentLeft" => Ok(TimeMachinePartType::TopRightTangentLeft),
            "BottomRightTangentLeft" => Ok(TimeMachinePartType::BottomRightTangentLeft),
            "TopLeftTangentBottom" => Ok(TimeMachinePartType::TopLeftTangentBottom),
            "BottomLeftTangentTop" => Ok(TimeMachinePartType::BottomLeftTangentTop),            
            "TopRightTangentBottom" => Ok(TimeMachinePartType::TopRightTangentBottom),
            "BottomRightTangentTop" => Ok(TimeMachinePartType::BottomRightTangentTop),

            "MiddleTopOpen" => Ok(TimeMachinePartType::MiddleTopOpen),
            "MiddleBottomOpen" => Ok(TimeMachinePartType::MiddleBottomOpen),
            "MiddleLeftOpen" => Ok(TimeMachinePartType::MiddleLeftOpen),
            "MiddleRightOpen" => Ok(TimeMachinePartType::MiddleRightOpen),
            _ => Err(())
        }
    }
}

#[derive(Default, Component)]
pub struct TimeMachineId(pub usize);

#[derive(Bundle, LdtkEntity)]
pub struct TimeMachinePartBundle {
    #[with(TimeMachinePartType::get_type)]
    part_type: TimeMachinePartType,
    #[with(GridEntityInfo::time_machine)]
    grid_entity: GridEntityInfo,
    size: TimeMachineGrid,
    #[grid_coords]
    position: GridCoords,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle
}

#[derive(Default, Component)]
pub struct TimeMachineGrid {
    pub id: usize,
    pub grid: Vec<Vec<(TimeMachinePartType, usize)>>
}