mod game;
mod player;
//mod map;
mod button;
mod cell;
mod cellMap;
mod directions;
mod labyrinth;
mod settings;

use crate::cellMap::CellMap;
use game::*;
use macroquad::prelude::*;
use std::time::Duration;
use std::{thread, vec};

fn window_config() -> Conf {
    Conf {
        window_title: settings::window::TITLE.to_string(),
        window_width: settings::window::WIDTH,
        window_height: settings::window::HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut game = Game::new(settings::menu::map::ROWS, settings::menu::map::COLS);
    let font = load_ttf_font("media/Akzidenz_Grotesk_Next_Bold.otf")
        .await
        .unwrap();
    rand::srand(miniquad::date::now() as u64);
    game.map_mut().gen_labyrinth();
    let mut next_trigger = get_time() + 0.5f64;
    loop {
        clear_background(settings::ui::BACKGROUND_COLOR);
        game.update(&font, &mut next_trigger);
        next_frame().await;
        thread::sleep(Duration::from_millis(20));
    }
}
