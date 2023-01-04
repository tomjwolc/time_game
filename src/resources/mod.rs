pub use super::*;

mod grid;
pub use grid::*;

#[derive(Resource)]
pub struct Ticks(pub usize);

#[derive(Resource)]
pub struct Dims {
    pub x: usize,
    pub y: usize
}