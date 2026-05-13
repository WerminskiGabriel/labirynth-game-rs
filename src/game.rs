use crate::cell::Cell;
use crate::cellMap::CellMap;
use crate::settings;
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
    pub fn new(size: usize) -> Self {
        Self {
            state: GameState::Menu,
            map: CellMap::new(size),
        }
    }
}

impl Game {
    pub fn update(&mut self) {
        match self.state {
            GameState::Playing => {}
            GameState::Menu => {
                if draw_button(Vec2::new(1000f32,50f32),200f32,200f32,"ENTER THE MAZE"){
                    self.state = GameState::Playing;
                }
            }
            GameState::Death => {}
            GameState::Settings => {}
            GameState::Completed => {}
        }
    }

    pub fn draw(&self) {
        match self.state {
            GameState::Playing => {}
            GameState::Menu => {
                self.map().draw(
                    settings::menu::cell::SIZE,
                    settings::menu::cell::THICKNESS,
                    settings::menu::map::START_POS,
                );
            }
            GameState::Death => {}
            GameState::Settings => {}
            GameState::Completed => {}
        }
    }
}

pub fn draw_button(vec: Vec2, w: f32, h: f32, text: &str) -> bool {
    let mouse_pos = mouse_position();
    let mut button_color = settings::button::COLOR;

    let is_hover = mouse_pos.0 >= vec.x
        && mouse_pos.0 <= vec.x + w
        && mouse_pos.1 >= vec.y
        && mouse_pos.1 <= vec.y + h;

    if is_hover {
        button_color = RED;
    }

    draw_rectangle(vec.x, vec.y, w, h, button_color);
    draw_text(text, vec.x + 20f32
              , vec.y + h / 2.0, settings::ui::FONT_SIZE as f32, WHITE);

    is_hover && is_mouse_button_pressed(MouseButton::Left)
}
