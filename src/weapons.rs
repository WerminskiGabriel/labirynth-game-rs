use crate::Bullet::Bullet;
use crate::cellMap::CellMap;
use crate::enemy::Enemy;
use crate::settings;
use macroquad::color::Color;
use macroquad::input::{MouseButton, is_key_pressed, is_mouse_button_down};
use macroquad::math::Vec2;
use macroquad::prelude::KeyCode;
use crate::sprites::Sprites;

enum Weapon {
    Gun,
    Sword,
}

struct WeaponBase {
    position: Vec2,
    width: f32,
    height: f32,
    offset: Vec2,
    color: Color,
}
pub trait Drawable {
    fn draw(&self, mouse_pos: Vec2);
}

pub struct Gun {
    base: WeaponBase,
    dmg: f32,
    bullet_speed: f32,
    bullets: Vec<Bullet>,
}

impl Gun {
    pub fn new() -> Self {
        Self {
            base: WeaponBase {
                position: Vec2::new(
                    settings::window::WIDTH as f32 / 2f32,
                    settings::window::HEIGHT as f32 / 2f32,
                ),
                width: settings::player::WIDTH,
                height: settings::player::HEIGHT,
                offset: Vec2::new(0f32, 0f32),
                color: settings::player::COLOR,
            },

            dmg: 10f32,
            bullet_speed: settings::player::SPEED,
            bullets: Vec::new(),
        }
    }
}
impl Gun {
    pub fn update(
        &mut self,
        mouse_pos: Vec2,
        player_pos: &Vec2,
        map: &CellMap,
        mut enemies: &mut Vec<Enemy>,
    ) {
        if is_mouse_button_down(MouseButton::Left) || is_key_pressed(KeyCode::Space) {
            let gun_position = Vec2::new(
                settings::window::WIDTH as f32 / 2f32,
                settings::window::HEIGHT as f32 / 2f32,
            );
            let rotation_vec = mouse_pos - gun_position;
            self.bullets.push(Bullet::new(*player_pos, rotation_vec));
        }

        self.bullets.retain_mut(|bullet| {
            !bullet.movement(map, &mut enemies)
        });
    }

    pub fn draw(&self, mouse_pos: Vec2, player_pos: &Vec2, sprites : &Sprites) {
        let rotation_vec = mouse_pos - self.base.position;
        let _rotation = rotation_vec.y.atan2(rotation_vec.x);

        let bullets = &self.bullets;
        for bullet in bullets {
            bullet.draw(player_pos, sprites);
        }

        /*
        draw_rectangle_ex(
            self.base.position.x,
            self.base.position.y,
            self.base.height,
            self.base.width,
            DrawRectangleParams {
                offset: self.base.offset,
                rotation,
                color: self.base.color,
            },
        );*/
    }
}
