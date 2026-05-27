use crate::cell::Cell;
use crate::cellMap::CellMap;
use crate::death::Death;
use crate::enemy::{Enemy, gen_new_enemies};
use crate::game::GameState::Completed;
use crate::labyrinth::fill_path_to_finish;
use crate::player::Player;
use crate::sprites::Sprites;
use crate::weapons::Drawable;
use crate::weapons::Gun;
use crate::{button, settings};
use getset::{Getters, MutGetters};
use macroquad::prelude::*;

enum GameState {
    Menu,
    Settings,
    Death,
    Completed,
    Playing,
}

#[derive(Getters, MutGetters)]
pub struct Game {
    #[getset(get = "pub", get_mut = "pub")]
    state: GameState,
    #[getset(get = "pub", get_mut = "pub")]
    map: CellMap,
    player: Player,
    gun: Gun,
    enemies: Vec<Enemy>,
    sprites: Sprites,
    deaths: Vec<Death>,
}

impl Game {
    pub fn new(cols: usize, rows: usize, sprites: Sprites) -> Self {
        Self {
            state: GameState::Menu,
            map: CellMap::new(cols, rows),
            player: Player::new(),
            gun: Gun::new(),
            enemies: gen_new_enemies(),
            sprites,
            deaths: vec![],
        }
    }
}

impl Game {
    pub fn update(&mut self, font: &Font, trigger: &mut f64) {
        match self.state {
            GameState::Playing => {
                let mouse_pos = mouse_position();
                let mouse_pos = Vec2::new(mouse_pos.0, mouse_pos.1);
                let ft = get_frame_time();

                self.map.draw_playing(&self.player, &self.sprites);

                self.gun.update(
                    //Vec2::new(mouse_pos.0, mouse_pos.1),
                    mouse_pos,
                    self.player.position(),
                    &self.map,
                    &mut self.enemies,
                );
                self.enemies.retain(|enemy| {
                    if *enemy.hp() <= 0f32 {
                        self.deaths.push(Death::new(enemy.position()));
                        return false;
                    }
                    true
                });
                {
                    self.deaths.retain_mut(|death| {
                        if death.update(ft) {
                        }

                        death.animation_frame <= settings::death::MAX_FRAMES
                    })
                }
                for death in &self.deaths {
                    death.draw(self.player.position(), &self.sprites);
                }

                self.gun.draw(mouse_pos, self.player.position());

                self.player.draw(&self.sprites, mouse_pos);
                self.player.update(&self.map);
                if self.player.check_finish(&self.map) || !self.player.is_alive() {
                    self.state = GameState::Menu;
                }

                for enemy in &self.enemies {
                    enemy.draw(self.player.position(), &self.sprites);
                }

                for mut enemy in &mut self.enemies {
                    if enemy.update(self.player.position(), ft, &self.map) {
                        *self.player.hp_mut() -= settings::enemy::ghost::DMG;
                    }
                }
            }

            GameState::Menu => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    self.map.change_labyrinth();
                }
                
                self.map().draw_menu(
                    settings::menu::cell::SIZE,
                    settings::menu::cell::THICKNESS,
                    Vec2::new(
                        settings::map::menu::COLS as f32 / 2f32,
                        settings::map::menu::ROWS as f32 / 2f32,
                    ),
                );


                if button::draw_button(
                    Vec2::new(
                        settings::window::WIDTH as f32 / 2f32 - settings::menu::button::W / 2f32,
                        settings::window::HEIGHT as f32 / 2f32 - settings::menu::button::H / 2f32,
                    ),
                    settings::menu::button::W,
                    settings::menu::button::H,
                    "ENTER THE MAZE",
                    font.to_owned(),
                ) {
                    self.map
                        .change_w_h(settings::map::playing::COLS, settings::map::playing::ROWS);
                    self.map.change_labyrinth();
                    self.state = GameState::Playing;
                    self.map.change_labyrinth();
                    fill_path_to_finish(self.map_mut(), Vec2::new(0f32, 0f32));
                    self.map.gen_enemy_grid();
                }
                /*
                if *trigger <= get_time() {
                    self.map.change_labyrinth();
                    *trigger = get_time() + 0.5f64;
                }*/
            }
            GameState::Death => {}
            GameState::Settings => {}
            GameState::Completed => {}
        }
    }
}
