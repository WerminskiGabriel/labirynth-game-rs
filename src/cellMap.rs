use crate::cell::Cell;
use crate::labyrinth::*;
use getset::{Getters, MutGetters};
use macroquad::prelude::*;

#[derive(Getters, MutGetters)]
pub struct CellMap {
    #[getset(get = "pub", get_mut = "pub")]
    width: usize,
    #[getset(get = "pub", get_mut = "pub")]
    height: usize,
    grid: Vec<Cell>,
    #[getset(get = "pub", get_mut = "pub")]
    full : bool
}

impl CellMap {
    pub fn new() -> Self {
        let width = 12usize;
        let height = 12usize;
        Self {
            width,
            height,
            grid: vec![Cell::new(); width * height],
            full : false,
        }
    }
    pub fn grid(&self, vec: Vec2) -> Cell {
        return self.grid[vec.y as usize * self.width + vec.x as usize].clone();
    }
}

impl CellMap {
    pub fn update(&mut self) {}
    pub fn draw(&self) {}
}
