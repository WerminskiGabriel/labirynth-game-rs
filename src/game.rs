use macroquad::color::WHITE;
use macroquad::prelude::{draw_circle, screen_height, screen_width};

enum GameState{
    Menu,
    Settings,
    Death,
    Completed,
    Playing,
}

pub struct Game {
    state : GameState,
}

impl Game {
    pub fn new() -> Self {
        Self{
            state: GameState::Playing,
        }
    }
}

impl Game{
    pub fn update(&mut self) {
        match self.state {
            GameState::Playing =>{
            },
            GameState::Menu =>{},
            GameState::Death =>{},
            GameState::Settings =>{},
            GameState::Completed =>{},
        }
    }

    pub fn draw( & self){
        match self.state {
            GameState::Playing =>{
                draw_circle( screen_width()/2f32, screen_height()/2f32, 10f32, WHITE );
            },
            GameState::Menu =>{},
            GameState::Death =>{},
            GameState::Settings =>{},
            GameState::Completed =>{},
        }
    }
}