mod game;
mod player;
//mod map;
mod cell;
mod cellMap;
mod directions;
mod labyrinth;
mod settings;

use game::*;
use std::vec;

use crate::cellMap::CellMap;
use macroquad::prelude::*;

#[macroquad::main("labirynth")]
async fn main() {
    let board_size = settings::map::SIZE;
    let board_width = settings::map::WIDTH;
    let tile_size = board_width / board_size as f32;

    let mut game = Game::new(board_size);
    //rand::srand(miniquad::date::now() as u64);
    game.map_mut().gen_labyrinth();

    loop {
        game.update();
        game.draw(tile_size,tile_size * 0.07 , Vec2::new(50f32, 50f32));

        next_frame().await
    }
}
