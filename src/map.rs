use crate::labyrinth::*;
use getset::{Getters, MutGetters};
use macroquad::prelude::*;
#[derive(Getters, MutGetters)]
pub struct Map {
    #[getset(get = "pub", get_mut = "pub")]
    width: usize,
    #[getset(get = "pub", get_mut = "pub")]
    height: usize,
    grid: Vec<u32>,
}

impl Map {
    pub fn new_empty(&self) -> Self {
        Self {
            width: 12usize,
            height: 12usize,
            grid: vec![2; self.width * self.height],
        }
    }
    pub fn new(&mut self) -> Self {
        self.new_empty();
        gen_labyrinth(self)
    }
    pub fn grid(&self, vec: Vec2) -> u32 {
        return self.grid[vec.y as usize * self.width + vec.x as usize];
    }
}

impl Map {
    pub fn update(&mut self) {}
    pub fn draw(&self) {}
}
