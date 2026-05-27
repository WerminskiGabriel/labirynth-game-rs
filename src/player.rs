use crate::CellStep::CellStep;
use crate::cell::Cell;
use crate::cellMap::CellMap;
use crate::collisions::walls_collision;
use crate::directions::Directions;
use crate::settings;
use crate::sprites::Sprites;
use getset::{Getters, MutGetters};
use macroquad::input::KeyCode::S;
use macroquad::miniquad::start;
use macroquad::prelude::*;
use std::iter::Map;

#[derive(Getters, MutGetters, Clone, Debug)]
pub struct Player {
    #[getset(get = "pub", get_mut = "pub")]
    hp: f32,
    #[getset(get = "pub", get_mut = "pub")]
    position: Vec2,
    speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            hp: settings::player::HP,
            position: Vec2::new(
                settings::cell::playing::SIZE * settings::map::playing::COLS as f32 / 2f32,
                settings::cell::playing::SIZE * settings::map::playing::ROWS as f32 / 2f32,
            ),
            speed: settings::player::SPEED,
        }
    }
}

impl Player {
    pub fn update(&mut self, map: &CellMap) {
        let speed_fast = settings::player::SPEED_FAST;
        let speed_slow = settings::player::SPEED;

        let mut speed = speed_slow;
        if is_key_down(KeyCode::LeftShift) {
            speed = speed_fast;
        } else if is_key_released(KeyCode::LeftShift) {
            speed = speed_slow;
        }

        let cell_size = settings::cell::playing::SIZE;
        let wall_size = settings::cell::playing::THICKNESS;
        let window_width = settings::window::WIDTH as f32;
        let window_height = settings::window::HEIGHT as f32;
        let player_radius = settings::player::SIZE / 2f32;

        let mut tmp_x: f32 = self.position.x;
        let mut wall_collision = false;
        if is_key_down(KeyCode::A) {
            tmp_x -= speed;

            if walls_collision(
                &self.position,
                player_radius,
                &mut tmp_x,
                map,
                Directions::West,
                0f32,
            ) {
                wall_collision = true
            }

            /*
            self.check_walls_collisions(
                &mut tmp_x,
                &cell_size,
                &wall_size,
                map,
                Directions::West,
                player_radius,
            );*/
        } else if is_key_down(KeyCode::D) {
            tmp_x += speed;

            if walls_collision(
                &self.position,
                player_radius,
                &mut tmp_x,
                map,
                Directions::East,
                0f32,
            ) {
                wall_collision = true;
            }
            /* self.check_walls_collisions(
                &mut tmp_x,
                &cell_size,
                &wall_size,
                map,
                Directions::East,
                player_radius,
            );*/
        }
        self.position.x = tmp_x;

        let mut tmp_y: f32 = self.position.y;
        if is_key_down(KeyCode::W) {
            tmp_y -= speed;
            if walls_collision(
                &self.position,
                player_radius,
                &mut tmp_y,
                map,
                Directions::North,
                0f32,
            ) {
                wall_collision = true;
            }
        /*
        self.check_walls_collisions(
            &mut tmp_y,
            &cell_size,
            &wall_size,
            map,
            Directions::North,
            player_radius,
        );*/
        } else if is_key_down(KeyCode::S) {
            tmp_y += speed;
            if walls_collision(
                &self.position,
                player_radius,
                &mut tmp_y,
                map,
                Directions::South,
                0f32,
            ) {
                wall_collision = true;
            }
            /*
            self.check_walls_collisions(
                &mut tmp_y,
                &cell_size,
                &wall_size,
                map,
                Directions::South,
                player_radius,
            );*/
        }
        if wall_collision {
            self.hp -= settings::cell::playing::DMG;
        }
        self.position.y = tmp_y
    }
    pub fn is_alive(&self) -> bool {
        return self.hp >= 0f32;
    }

    pub fn draw(&self, sprites: &Sprites, mouse_pos: Vec2) {
        let screen_center = Vec2::new(
            settings::window::WIDTH as f32 / 2f32,
            settings::window::HEIGHT as f32 / 2f32,
        );

        let dest_size = Vec2::new(settings::player::SIZE, settings::player::SIZE) * 5f32;

        let player_pos = Vec2::new(
            settings::window::WIDTH as f32 / 2f32,
            settings::window::HEIGHT as f32 / 2f32,
        );

        let rotation_vec = mouse_pos - player_pos;
        let rotation = rotation_vec.y.atan2(rotation_vec.x) - std::f32::consts::PI / 2f32;

        let player_pos = screen_center - (dest_size / 2f32);

        draw_texture_ex(
            &sprites.player,
            player_pos.x,
            player_pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(dest_size),
                source: None,
                rotation: rotation,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
        /*
        draw_circle(
            settings::window::WIDTH as f32 / 2f32,
            settings::window::HEIGHT as f32 / 2f32,
            settings::player::SIZE,
            settings::player::COLOR,
        );*/

        Self::draw_hp_bar(self);

        draw_text(
            format!("{}", self.position).as_str(),
            20f32,
            20f32,
            20f32,
            BLACK,
        );
    }
    pub fn check_finish(&self, map: &CellMap) -> bool {
        let cell_size = settings::cell::playing::SIZE;

        return *map
            .grid(Vec2::new(
                self.position().x / cell_size,
                self.position.y / cell_size,
            ))
            .step()
            == CellStep::Finish;
    }

    fn draw_hp_bar(&self) {
        let thickness = settings::player::SIZE / 5f32;
        draw_rectangle_lines(
            settings::window::WIDTH as f32 / 2f32 - settings::player::SIZE,
            settings::window::HEIGHT as f32 / 2f32 + settings::player::SIZE * 2f32,
            settings::player::SIZE * 2f32,
            settings::player::SIZE / 2f32,
            thickness,
            settings::player::COLOR,
        );
        draw_rectangle(
            settings::window::WIDTH as f32 / 2f32 - settings::player::SIZE + thickness / 2f32,
            settings::window::HEIGHT as f32 / 2f32
                + settings::player::SIZE * 2f32
                + thickness / 2f32,
            (settings::player::SIZE * 2f32 - thickness) * (self.hp / settings::player::HP),
            settings::player::SIZE / 2f32 - thickness,
            settings::hp_bar::COLOR,
        )
    }
}
