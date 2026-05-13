use crate::cell::Cell;
use crate::cellMap::CellMap;
use crate::settings;
use getset::{Getters, MutGetters};
use macroquad::color::WHITE;
use macroquad::math::Vec2;
use macroquad::prelude::{draw_circle, screen_height, screen_width};
use macroquad::window::clear_background;

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
    pub fn new(size: usize) -> Self {
        Self {
            state: GameState::Playing,
            map: CellMap::new(size),
        }
    }
}

impl Game {
    pub fn update(&mut self) {
        match self.state {
            GameState::Playing => {}
            GameState::Menu => {}
            GameState::Death => {}
            GameState::Settings => {}
            GameState::Completed => {}
        }
    }

    pub fn draw(&self, tile_size: f32, wall_size: f32, start_pos_tl: Vec2) {
        match self.state {
            GameState::Playing => {
                clear_background(settings::ui::BACKGROUND_COLOR);
                self.map().draw(tile_size, wall_size, start_pos_tl);
            }
            GameState::Menu => {}
            GameState::Death => {}
            GameState::Settings => {}
            GameState::Completed => {}
        }
    }
}
