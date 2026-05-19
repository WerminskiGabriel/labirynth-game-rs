use crate::cellMap::CellMap;
use crate::collisions::walls_collision;
use crate::directions::Directions;
use crate::settings;
use macroquad::math::Vec2;
use macroquad::prelude::draw_circle;

pub struct Bullet {
    position: Vec2,
    rotation_vec: Vec2,
    lifetime: f32,
}

impl Bullet {
    pub fn new(spawn_position: Vec2, rotation_vec: Vec2) -> Self {
        Self {
            position: spawn_position,
            rotation_vec,
            lifetime: settings::bullet::LIFETIME,
        }
    }
    pub fn movement(&mut self, map: &CellMap) -> bool {
        let bullet_radius = settings::bullet::SIZE;
        let bullet_speed = settings::bullet::SPEED;
        let window_w = settings::window::WIDTH as f32;
        let window_h = settings::window::HEIGHT as f32;
        let offset = 0f32;
        let movement_vec = self.rotation_vec.normalize() * bullet_speed;

        let step_size = 5f32;
        let steps = (bullet_speed / step_size).floor() + 1f32;
        let step_movement_vec = movement_vec / steps;

        for idx in 0..steps as usize {
            let tmp_pos = self.position + step_movement_vec;
            
            let mut tmp_x = tmp_pos.x;
            let direction_x = if (tmp_x < window_w / 2f32) {
                Directions::West
            } else {
                Directions::East
            };
            if walls_collision(
                &self.position,
                bullet_radius,
                &mut tmp_x,
                &map,
                direction_x,
                offset,
            ) {
                return true;
            }

            let mut tmp_y = tmp_pos.y;
            let direction_y = if (tmp_y < window_h / 2f32) {
                Directions::North
            } else {
                Directions::South
            };
            if walls_collision(
                &self.position,
                bullet_radius,
                &mut tmp_y,
                map,
                direction_y,
                offset,
            ) {
                return true;
            }

            self.position = Vec2::new(tmp_x, tmp_y);
        }
        false
    }
    pub fn draw(&self, player_pos: &Vec2) {
        let window_w = settings::window::WIDTH as f32;
        let window_h = settings::window::HEIGHT as f32;
        let offset = 10f32;

        let bullet_camera_x = self.position.x - player_pos.x + window_w / 2f32;
        let bullet_camera_y = self.position.y - player_pos.y + window_h / 2f32;

        if -offset <= bullet_camera_x
            && bullet_camera_x - window_w <= offset
            && -offset <= bullet_camera_y
            && bullet_camera_y - window_h <= offset
        {
            draw_circle(
                bullet_camera_x,
                bullet_camera_y,
                settings::bullet::SIZE,
                settings::bullet::COLOR,
            );
        }
    }
}
