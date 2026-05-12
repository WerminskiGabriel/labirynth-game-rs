use macroquad::prelude::*;
use crate::game::Game;

pub struct Player{
    HP : i32,
    position : Vec2,
}

impl Player{
    pub fn new() -> Self {
        Self {
            HP : 100,
            position : Vec2::new(0f32,0f32),
        }
    }
}

impl Player{
    pub fn update(&mut self) {

    }
    pub fn draw( & self){

    }
}