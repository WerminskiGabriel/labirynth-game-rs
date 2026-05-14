use crate::cell::Cell;
use crate::cellMap::CellMap;
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
}

impl Game {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            state: GameState::Menu,
            map: CellMap::new(cols, rows),
        }
    }
}

impl Game {
    pub fn update(&mut self, font: &Font, trigger : &mut f64) {
        match self.state {
            GameState::Playing => {}
            GameState::Menu => {
                self.map().draw(
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
                } else if *trigger <= get_time(){
                    self.map.change_labyrinth();
                    *trigger = get_time() + 2f64;
                
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
