use ::rand::random_range;
use crate::CellStep::CellStep;
use crate::cellMap::CellMap;
use crate::directions::Directions;
use crate::settings;
use crate::sprites::Sprites;
use getset::{Getters, MutGetters};
use macroquad::prelude::*;

pub enum EnemyType {
    Ghost,
    Goblin,
}

#[derive(Getters, MutGetters)]
pub struct Enemy {
    #[getset(get = "pub", get_mut = "pub")]
    position: Vec2,
    #[getset(get = "pub", get_mut = "pub")]
    hp: f32,
    dmg_cooldown: f32,
    #[getset(get = "pub", get_mut = "pub")]
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
                self.position += movement_vec.normalize() * settings::enemy::ghost::SPEED * ft;
            }
            EnemyType::Goblin => {
                self.dmg_cooldown -= ft;

                let move_speed = settings::enemy::goblin::SPEED;
                let cell_size = settings::cell::playing::SIZE;

                let enemy_idx = (self.position / settings::cell::playing::SIZE).floor();
                let player_idx = (*player_pos / settings::cell::playing::SIZE).floor();

                let curr_cell = map.enemy_grid(enemy_idx, player_idx);
                let movement_idx = match curr_cell {
                    CellStep::Direction(dir) => match dir {
                        Directions::North => Vec2::new(0f32, -1f32),
                        Directions::East => Vec2::new(1f32, 0f32),
                        Directions::South => Vec2::new(0f32, 1f32),
                        Directions::West => Vec2::new(-1f32, 0f32),
                    },
                    _ => Vec2::new(0f32, 0f32),
                };

                let dest_cell_center_pos = match curr_cell {
                    CellStep::Finish => *player_pos,
                    CellStep::Unvisited => *player_pos,
                    _ => {
                        ((enemy_idx + movement_idx) * settings::cell::playing::SIZE
                            + Vec2::new(
                                settings::cell::playing::SIZE / 2f32,
                                settings::cell::playing::SIZE / 2f32,
                            ))
                    }
                };

                let movement_vec = (dest_cell_center_pos - self.position);

                if movement_vec.length_squared()
                    <= (settings::enemy::goblin::SIZE * settings::enemy::goblin::SIZE)
                    && self.dmg_cooldown <= 0f32
                {
                    self.dmg_cooldown = settings::enemy::goblin::COOLDOWN;
                    return true;
                }

                self.position += movement_vec.normalize_or_zero() * move_speed * ft;
            }
        };

        return false;
    }
    pub fn draw(&self, player_pos: &Vec2, sprite: &Sprites) {
        let window_w = settings::window::WIDTH as f32;
        let window_h = settings::window::HEIGHT as f32;
        let offset = 10f32;

        let dest_size = match self.enemy_type {
            EnemyType::Ghost => Vec2::new(
                settings::enemy::ghost::SIZE * 2f32,
                settings::enemy::ghost::SIZE * 2f32,
            ),
            EnemyType::Goblin => Vec2::new(
                settings::enemy::goblin::SIZE * 2f32,
                settings::enemy::goblin::SIZE * 2f32,
            ),
        };

        let bullet_camera_x = self.position.x - player_pos.x + window_w / 2f32;
        let bullet_camera_y = self.position.y - player_pos.y + window_h / 2f32;

        //Self::draw_hp_bar(self, &bullet_camera_x, &bullet_camera_y);

        let dest_sprite = match self.enemy_type {
            EnemyType::Ghost => &sprite.enemies.ghost,
            EnemyType::Goblin => &sprite.enemies.goblin,
        };

        if -offset <= bullet_camera_x
            && bullet_camera_x - window_w <= offset
            && -offset <= bullet_camera_y
            && bullet_camera_y - window_h <= offset
        {
            draw_texture_ex(
                &dest_sprite,
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
    fn draw_hp_bar(&self, pos_x: &f32, pos_y: &f32) {
        let size = match self.enemy_type {
            EnemyType::Ghost => settings::enemy::ghost::SIZE,
            EnemyType::Goblin => settings::enemy::goblin::SIZE,
        };
        let thickness = size / 5f32;
        let hp_max = match self.enemy_type {
            EnemyType::Ghost => settings::enemy::ghost::HP,
            EnemyType::Goblin => settings::enemy::goblin::HP,
        };

        draw_rectangle_lines(
            pos_x - size,
            pos_y + size * 2f32,
            size * 2f32,
            size / 2f32,
            thickness,
            settings::player::COLOR,
        );
        let curr_hp = if self.hp >= 0f32 { self.hp } else { 0f32 };
        draw_rectangle(
            pos_x - size + thickness / 2f32,
            pos_y + size * 2f32 + thickness / 2f32,
            (size * 2f32 - thickness) * (curr_hp / hp_max),
            size / 2f32 - thickness,
            settings::hp_bar::COLOR,
        )
    }
}
pub fn gen_new_enemies() -> Vec<Enemy> {
    let mut enemies: Vec<Enemy> = vec![];

    let tile_size = settings::cell::playing::SIZE;

    for _ in 0..settings::enemy::MAX_COUNT {
        let x = random_range(1..settings::map::playing::COLS-1) as f32 * tile_size
            - tile_size / 2f32;
        let y = random_range(1..settings::map::playing::ROWS-1) as f32 * tile_size
            - tile_size / 2f32;

        if  random_range(0..=100) < (settings::enemy::GOBLIN_GHOST_RATIO * 100f32 ) as i32{
            enemies.push(Enemy::new(Vec2::new(x, y), EnemyType::Goblin));
        }else {
            enemies.push(Enemy::new(Vec2::new(x, y), EnemyType::Ghost));
        }

    }

    return enemies;
}
