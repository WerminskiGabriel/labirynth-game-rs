use crate::CellStep::CellStep;
use crate::cell::Cell;
use crate::cellMap::CellMap;
use crate::directions::Directions;
use crate::settings;
use macroquad::color::RED;
use macroquad::math::Vec2;
use macroquad::prelude::draw_circle;
use rand::random;
use std::arch::x86_64::_mm_or_epi32;

pub enum EnemyType {
    Ghost,
    Goblin,
}

pub struct Enemy {
    position: Vec2,
    hp: f32,
    dmg_cooldown: f32,
    enemy_type: EnemyType,
}

impl Enemy {
    pub fn new(position: Vec2, enemy_type: EnemyType) -> Self {
        Self {
            position,
            hp: match enemy_type {
                EnemyType::Ghost => settings::enemy::ghost::HP,
                EnemyType::Goblin => settings::enemy::goblin::HP,
            },
            dmg_cooldown: match enemy_type {
                EnemyType::Ghost => settings::enemy::ghost::COOLDOWN,
                EnemyType::Goblin => settings::enemy::goblin::COOLDOWN,
            },
            enemy_type,
        }
    }

    pub fn update(&mut self, player_pos: &Vec2, ft: f32, map: &CellMap) -> bool {
        // returns true if collision occurred and false elsewhere
        match self.enemy_type {
            EnemyType::Ghost => {
                self.dmg_cooldown -= ft;
                let movement_vec = (*player_pos - self.position);

                if movement_vec.length_squared()
                    <= (settings::enemy::ghost::SIZE * settings::enemy::ghost::SIZE)
                    && self.dmg_cooldown <= 0f32
                {
                    self.dmg_cooldown = settings::enemy::ghost::COOLDOWN;
                    return true;
                }
                self.position += movement_vec.normalize() * settings::enemy::ghost::SPEED;
            }
            EnemyType::Goblin => {
                self.dmg_cooldown -= ft;

                let move_speed = settings::enemy::goblin::SPEED;

                let enemy_idx = self.position / settings::cell::playing::SIZE;
                let player_idx = *player_pos / settings::cell::playing::SIZE;

                let curr_cell = map.enemy_grid(enemy_idx, player_idx);

                let movement_vec = match curr_cell {
                    CellStep::Direction(dir) => match dir {
                        Directions::North => Vec2::new(0f32, -move_speed),
                        Directions::East => Vec2::new(move_speed, 0f32),
                        Directions::South => Vec2::new(0f32, move_speed),
                        Directions::West => Vec2::new(-move_speed, 0f32),
                    },
                    _ => Vec2::new(0f32, 0f32),
                };

                if movement_vec.length_squared()
                    <= (settings::enemy::goblin::SIZE * settings::enemy::goblin::SIZE)
                    && self.dmg_cooldown <= 0f32
                {   self.dmg_cooldown = settings::enemy::goblin::COOLDOWN;
                    return true;
                }

                self.position += movement_vec;
            }
        };

        return false;
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
                settings::enemy::ghost::SIZE,
                RED,
            );
        }
    }
}

pub fn gen_new_enemies() -> Vec<Enemy> {
    let mut enemies: Vec<Enemy> = vec![];

    let tile_size = settings::cell::playing::SIZE;

    for _ in 0..settings::enemy::MAX_COUNT {
        let x = rand::random_range(0..settings::map::playing::COLS) as f32 * tile_size
            - tile_size / 2f32;
        let y = rand::random_range(0..settings::map::playing::ROWS) as f32 * tile_size
            - tile_size / 2f32;
        enemies.push(Enemy::new(Vec2::new(x, y), EnemyType::Goblin));
    }

    return enemies;
}
