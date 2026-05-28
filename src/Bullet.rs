use crate::cellMap::CellMap;
use crate::collisions::walls_collision;
use crate::directions::Directions;
use crate::enemy::{Enemy, EnemyType};
use crate::settings;
use crate::sprites::{Enemies, Sprites};
use macroquad::math::Vec2;
use macroquad::prelude::{draw_circle, draw_texture_ex, DrawTextureParams};
use std::any::Any;
use macroquad::color::WHITE;

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
    pub fn movement(&mut self, map: &CellMap, enemies: &mut Vec<Enemy>) -> bool {
        let bullet_radius = settings::bullet::SIZE;
        let bullet_speed = settings::bullet::SPEED;
        let window_w = settings::window::WIDTH as f32;
        let window_h = settings::window::HEIGHT as f32;
        let offset = 0f32;
        let movement_vec = self.rotation_vec.normalize() * bullet_speed;

        let step_size = 5f32;
        let steps = (bullet_speed / step_size).floor() + 1f32;
        let step_movement_vec = movement_vec / steps;
        let mut is_collision: bool = false;

        for idx in 0..steps as usize {
            let tmp_pos = self.position + step_movement_vec;

            {
                // walls collisions
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
                    is_collision = true;
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
                    is_collision = true;
                }

                self.position = Vec2::new(tmp_x, tmp_y);
            }
            {
                //enemies collisions
                for mut enemy in &mut *enemies {
                    let enemy_radius = match enemy.enemy_type() {
                        EnemyType::Ghost => settings::enemy::ghost::SIZE,
                        EnemyType::Goblin => settings::enemy::goblin::SIZE,
                    };
                    let bullet_radius = bullet_radius;
                    let enemy_pos = *enemy.position();
                    let bullet_pos = self.position;
                    let tmp_bullet_pos = tmp_pos;

                    let distance = bullet_pos.distance(enemy_pos);

                    if distance < bullet_radius + enemy_radius{
                        is_collision = true;
                        *enemy.hp_mut() -= settings::player::DMG;
                    }
                }
            }
        }
        is_collision
    }
    pub fn draw(&self, player_pos: &Vec2, sprites: &Sprites) {
        let window_w = settings::window::WIDTH as f32;
        let window_h = settings::window::HEIGHT as f32;
        let offset = 10f32;

        let bullet_camera_x = self.position.x - player_pos.x + window_w / 2f32;
        let bullet_camera_y = self.position.y - player_pos.y + window_h / 2f32;

        let dest_size = Vec2::new( settings::bullet::SIZE*2f32, settings::bullet::SIZE*2f32);

        if -offset <= bullet_camera_x
            && bullet_camera_x - window_w <= offset
            && -offset <= bullet_camera_y
            && bullet_camera_y - window_h <= offset
        {
           

            draw_texture_ex(
                &sprites.bullet,
                bullet_camera_x - dest_size.x / 2f32,
                bullet_camera_y - dest_size.y / 2f32,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(dest_size),
                    source: None,
                    rotation: 0f32,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                },
            );
        }
    }
}
