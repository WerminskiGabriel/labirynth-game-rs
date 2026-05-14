mod game;
mod player;
//mod map;
mod cell;
mod cellMap;
mod directions;
mod labyrinth;
mod settings;
mod button;

use game::*;
use std::{thread, vec};
use std::time::Duration;
use crate::cellMap::CellMap;
use macroquad::prelude::*;

fn window_config( ) -> Conf {
    Conf{
        window_title: settings::window::TITLE.to_string(),
        window_width: settings::window::WIDTH,
        window_height: settings::window::HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut game = Game::new(settings::menu::map::ROWS,settings::menu::map::COLS);
    let font = load_ttf_font("media/Akzidenz_Grotesk_Next_Bold.otf").await.unwrap();
    rand::srand(miniquad::date::now() as u64);
    game.map_mut().gen_labyrinth();
    let mut next_trigger = get_time() + 0.0f64;
    loop {
        clear_background(settings::ui::BACKGROUND_COLOR);
        game.update(&font, &mut next_trigger);
        next_frame().await;
        thread::sleep(Duration::from_millis(20));
    }
}
