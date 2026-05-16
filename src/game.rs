use crate::cell::Cell;
use crate::cellMap::CellMap;
use crate::player::Player;
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
}

impl Game {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            state: GameState::Menu,
            map: CellMap::new(cols, rows),
            player: Player::new(),
        }
    }
}

impl Game {
    pub fn update(&mut self, font: &Font, trigger: &mut f64) {
        match self.state {
            GameState::Playing => {
                self.map.draw_playing(
                    &self.player,
                    settings::playing::cell::SIZE,
                    settings::playing::cell::THICKNESS,
                    settings::window::WIDTH as f32,
                    settings::window::HEIGHT as f32,
                );
                self.player.draw();
                self.player.update(&self.map);
            }
            GameState::Menu => {
                self.map().draw_menu(
                    settings::menu::cell::SIZE,
                    settings::menu::cell::THICKNESS,
                    settings::menu::map::START_POS,
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
                    self.state = GameState::Playing;
                }
                if *trigger <= get_time() {
                    self.map.change_labyrinth();
                    *trigger = get_time() + 0.5f64;
                }

                /*if is_mouse_button_pressed(MouseButton::Left) {
                    self.map.change_labyrinth();
                }*/
            }
            GameState::Death => {}
            GameState::Settings => {}
            GameState::Completed => {}
        }
    }
}
