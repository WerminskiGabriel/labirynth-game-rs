mod game;
mod player;
//mod map;
mod Settings;
mod cell;
mod cellMap;
mod directions;
mod labyrinth;

use std::vec;
use game::*;

use crate::cellMap::CellMap;
use macroquad::prelude::*;

#[macroquad::main("labirynth")]
async fn main() {
    let mut game = Game::new();

    let map = CellMap::new();
    print!("{:?}", map.grid(Vec2::new(2f32,2f32)));
    loop {


        //game.update();
        //game.draw();

        next_frame().await
    }
}
