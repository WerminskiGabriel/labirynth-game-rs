use crate::cell::Cell;
use crate::labyrinth::*;
use getset::{Getters, MutGetters};
use macroquad::prelude::*;
use crate::settings;
#[derive(Getters, MutGetters)]
pub struct CellMap {
    #[getset(get = "pub", get_mut = "pub")]
    width: usize,
    #[getset(get = "pub", get_mut = "pub")]
    height: usize,
    grid: Vec<Cell>,
}

impl CellMap {
    pub fn new(cols: usize, rows: usize) -> Self {
        let width = cols;
        let height = rows;
        Self {
            width,
            height,
            grid: vec![Cell::new(); width * height],
        }
    }
    pub fn gen_labyrinth(&mut self) {
        gen_labyrinth(self);
    }
    pub fn change_labyrinth(&mut self) {
        self.grid = vec![Cell::new(); self.width * self.height];
        self.gen_labyrinth();
    }

    pub fn grid(&self, vec: Vec2) -> &Cell {
        &self.grid[vec.y as usize * self.width + vec.x as usize]
    }
    pub fn grid_mut(&mut self, vec: Vec2) -> &mut Cell {
        &mut self.grid[vec.y as usize * self.width + vec.x as usize]
    }
}

impl CellMap {
    pub fn update(&mut self) {}
    pub fn draw(&self, tile_size: f32, wall_size: f32, start_pos_tl: Vec2) {
        draw_line(
            start_pos_tl.x,
            start_pos_tl.y,
            start_pos_tl.x,
            start_pos_tl.y + tile_size * self.height as f32,
            wall_size,
            settings::menu::cell::COLOR,
        );
        draw_line(
            start_pos_tl.x,
            start_pos_tl.y,
            start_pos_tl.x + tile_size * self.width as f32,
            start_pos_tl.y,
            wall_size,
            settings::menu::cell::COLOR,
        );

        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.grid(Vec2::new(x as f32, y as f32));

                let x_pos = start_pos_tl.x + x as f32 * tile_size;
                let y_pos = start_pos_tl.y + y as f32 * tile_size;

                cell.draw(&x_pos, &y_pos, &tile_size, &wall_size);
            }
        }
    }
}
