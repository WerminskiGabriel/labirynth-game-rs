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
    let mut game = Game::new(settings::menu::map::SIZE);
    rand::srand(miniquad::date::now() as u64);
    game.map_mut().gen_labyrinth();

    loop {
        clear_background(settings::ui::BACKGROUND_COLOR);
        game.update();
        game.draw();
        next_frame().await
    }
}
