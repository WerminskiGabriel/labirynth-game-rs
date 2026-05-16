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
    speed : f32,
}

impl Player {
    pub fn new() -> Self {
        let spawn = settings::playing::cell::SIZE / 2f32;
        Self {
            HP: 100,
            position: Vec2::new(spawn, spawn),
            speed : settings::player::SPEED,
        }
    }
}

impl Player {
    pub fn update(&mut self, map: &CellMap) {
        let speed_fast = settings::player::SPEED_FAST;
        let speed_slow = settings::player::SPEED;

        let mut speed = speed_slow;
        if is_key_down(KeyCode::LeftShift){
            speed = speed_fast;
        }else if is_key_released(KeyCode::LeftShift){
            speed = speed_slow;
        }

        let cell_size = settings::playing::cell::SIZE;
        let wall_size = settings::playing::cell::THICKNESS;
        let window_width = settings::window::WIDTH as f32;
        let window_height = settings::window::HEIGHT as f32;
        let player_radius = settings::player::SIZE / 2f32;



        let mut tmp_x: f32 = self.position.x;

        if is_key_down(KeyCode::A) {
            tmp_x -= speed;

            self.check_walls_collisions(
                &mut tmp_x,
                &cell_size,
                &wall_size,
                map,
                Directions::West,
                player_radius,
            );
        } else if is_key_down(KeyCode::D) {
            tmp_x += speed;

            self.check_walls_collisions(
                &mut tmp_x,
                &cell_size,
                &wall_size,
                map,
                Directions::East,
                player_radius,
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
                player_radius,
            );
        } else if is_key_down(KeyCode::S) {
            tmp_y += speed;
            self.check_walls_collisions(
                &mut tmp_y,
                &cell_size,
                &wall_size,
                map,
                Directions::South,
                player_radius,
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
        player_radius: f32,
    ) {
        let start_x_idx: f32 = (self.position.x / cell_size).floor();
        let start_y_idx = (self.position.y / cell_size).floor();

        let vec = Vec2::new(start_x_idx, start_y_idx);
        let cell = map.grid(vec);

        match direction {
            Directions::North => {
                if cell.is_north_wall()
                    && ((*tmp - player_radius - wall_size) / cell_size).floor() != start_y_idx
                {
                    *tmp = start_y_idx * cell_size + wall_size + player_radius;
                }
            }
            Directions::East => {
                if cell.is_east_wall()
                    && ((*tmp + player_radius + wall_size) / cell_size).floor() != start_x_idx
                {
                    *tmp = (start_x_idx + 1f32) * cell_size - wall_size - player_radius
                }
            }
            Directions::South => {
                if cell.is_south_wall()
                    && ((*tmp + player_radius+ wall_size) / cell_size).floor() != start_y_idx
                {
                    *tmp = (start_y_idx + 1f32) * cell_size - wall_size - player_radius
                }
            }
            Directions::West => {
                if cell.is_west_wall()
                    && ((*tmp - player_radius - wall_size) / cell_size).floor() != start_x_idx
                {
                    *tmp = start_x_idx * cell_size + wall_size + player_radius
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
