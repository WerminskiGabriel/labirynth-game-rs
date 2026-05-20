use crate::cell::Cell;
use crate::cellMap::CellMap;
use crate::player::Player;
use crate::weapons::Gun;
use crate::{button, settings};
use getset::{Getters, MutGetters};
use macroquad::prelude::*;
use crate::game::GameState::Completed;
use crate::labyrinth::fill_path_to_finish;
use crate::weapons::Drawable;

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
    player: Player,
    gun: Gun,
}

impl Game {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            state: GameState::Playing,
            map: CellMap::new(cols, rows),
            player: Player::new(),
            gun: Gun::new(),
        }
    }
}

impl Game {
    pub fn update(&mut self, font: &Font, trigger: &mut f64) {
        match self.state {
            GameState::Playing => {
                let mouse_pos = mouse_position();

                self.map.draw_playing(&self.player);

                self.gun.update(Vec2::new(mouse_pos.0,mouse_pos.1), self.player.position() , &self.map );
                self.gun.draw(Vec2::new(mouse_pos.0,mouse_pos.1), self.player.position());

                self.player.draw();
                self.player.update(&self.map);
                if self.player.check_finish(&self.map) || !self.player.is_alive(){
                    self.state = GameState::Menu;
                }

            }

            GameState::Menu => {
                self.map().draw_menu(
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
                    self.map.change_w_h(settings::map::playing::COLS,settings::map::playing::ROWS);
                    self.map.change_labyrinth();
                    self.state = GameState::Playing;

                    self.map.change_labyrinth()
                }
                /*
                if *trigger <= get_time() {
                    self.map.change_labyrinth();
                    *trigger = get_time() + 0.5f64;
                }*/

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
