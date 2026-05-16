use crate::cell::Cell;
use crate::cellMap::CellMap;
use crate::directions::Directions;
use crate::settings;
use getset::{Getters, MutGetters};
use macroquad::input::KeyCode::S;
use macroquad::miniquad::start;
use macroquad::prelude::*;
use std::iter::Map;

#[derive(Getters, MutGetters, Clone, Debug)]
pub struct Player {
    #[getset(get = "pub", get_mut = "pub")]
    HP: i32,
    #[getset(get = "pub", get_mut = "pub")]
    position: Vec2,
}

impl Player {
    pub fn new() -> Self {
        let spawn = settings::playing::cell::SIZE / 2f32;
        Self {
            HP: 100,
            position: Vec2::new(spawn, spawn),
        }
    }
}

impl Player {
    pub fn update(&mut self, map: &CellMap) {
        let cell_size = settings::playing::cell::SIZE;
        let wall_size = settings::playing::cell::THICKNESS;
        let window_width = settings::window::WIDTH as f32;
        let window_height = settings::window::HEIGHT as f32;

        let speed = settings::player::SPEED;

        let mut tmp_x: f32 = self.position.x;

        if is_key_down(KeyCode::A) {
            tmp_x -= speed;

            self.check_walls_collisions(
                &mut tmp_x,
                &cell_size,
                &wall_size,
                map,
                Directions::West,
                true,
            );
        } else if is_key_down(KeyCode::D) {
            tmp_x += speed;

            self.check_walls_collisions(
                &mut tmp_x,
                &cell_size,
                &wall_size,
                map,
                Directions::East,
                true,
            );
        }
        self.position.x = tmp_x;

        let mut tmp_y: f32 = self.position.y;
        if is_key_down(KeyCode::W) {
            tmp_y -= speed;
            self.check_walls_collisions(
                &mut tmp_y,
                &cell_size,
                &wall_size,
                map,
                Directions::North,
                false,
            );
        } else if is_key_down(KeyCode::S) {
            tmp_y += speed;
            self.check_walls_collisions(
                &mut tmp_y,
                &cell_size,
                &wall_size,
                map,
                Directions::South,
                false,
            );
        }

        self.position.y = tmp_y
    }

    pub fn check_walls_collisions(
        &self,
        tmp: &mut f32,
        cell_size: &f32,
        wall_size: &f32,
        map: &CellMap,
        direction: Directions,
        is_x: bool,
    ) {
        let start_x_idx: f32 = (self.position.x / cell_size).floor();
        let start_y_idx = (self.position.y / cell_size).floor();

        let vec = Vec2::new(start_x_idx, start_y_idx);
        let cell = map.grid(vec);

        match direction {
            Directions::North => {
                if cell.is_north_wall() && (*tmp / cell_size).floor() != start_y_idx {
                    *tmp = start_y_idx * cell_size + wall_size
                }
            }
            Directions::East => {
                if cell.is_east_wall() && (*tmp / cell_size).floor() != start_x_idx {
                    *tmp =(start_x_idx + 1f32) * cell_size - wall_size
                }
            }
            Directions::South => {
                if cell.is_south_wall() && (*tmp / cell_size).floor() != start_y_idx {
                    *tmp = (start_y_idx + 1f32) * cell_size - wall_size
                }
            }
            Directions::West => {
                if cell.is_west_wall() && (*tmp / cell_size).floor() != start_x_idx {
                    *tmp = start_x_idx * cell_size + wall_size
                }
            }
        };
    }

    pub fn draw(&self) {
        draw_circle(
            settings::window::WIDTH as f32 / 2f32,
            settings::window::HEIGHT as f32 / 2f32,
            settings::player::SIZE,
            settings::player::COLOR,
        );
        draw_text(
            format!("{}", self.position).as_str(),
            20f32,
            20f32,
            20f32,
            BLACK,
        );
    }
}
