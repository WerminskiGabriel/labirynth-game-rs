use crate::CellStep::CellStep;
use crate::cellMap::CellMap;
use crate::directions::Directions;
use crate::settings;
use crate::sprites::Sprites;
use getset::{Getters, MutGetters};
use macroquad::prelude::*;
use std::any::Any;

#[derive(Getters, MutGetters, Clone, Debug)]
pub struct Cell {
    //#[getset(get = "pub", get_mut = "pub")]
    //directions: [Directions; 4],
    #[getset(get = "pub", get_mut = "pub")]
    walls_bit: u8,
    #[getset(get = "pub", get_mut = "pub")]
    step: CellStep,
    //#[getset(get="pub", get_mut="pub")]
    // is_visited : bool,
}
impl Cell {
    pub fn new() -> Self {
        Self {
            //    directions: Self::gen_directions(),
            walls_bit: 0b1111, //NESW
            step: CellStep::Unvisited,
            //is_visited: false,
        }
    }
    pub fn gen_directions() -> [Directions; 4] {
        let mut nums: [u8; 4] = [0, 1, 2, 3];
        let mut max_idx = 3i8;

        let mut new_directions: [Directions; 4] = [Directions::North; 4];
        let mut directions_idx = 0;

        while max_idx >= 0 {
            let rnd = rand::gen_range(0, (max_idx + 1) as usize);
            let new_direction_idx = nums[rnd];
            nums[rnd] = nums[max_idx as usize];
            max_idx -= 1;
            new_directions[directions_idx] = match new_direction_idx {
                0 => Directions::North,
                1 => Directions::East,
                2 => Directions::South,
                3 => Directions::West,
                _ => Directions::North, // Should not occur
            };
            directions_idx += 1;
        }
        return new_directions;
    }
}
impl Cell {
    pub fn remove_wall(&mut self, direction: Directions) {
        match direction {
            Directions::North => self.walls_bit &= 0b0111,
            Directions::East => self.walls_bit &= 0b1011,
            Directions::South => self.walls_bit &= 0b1101,
            Directions::West => self.walls_bit &= 0b1110,
        }
    }

    pub fn is_full_walls(&self) -> bool {
        self.walls_bit & 0b1111 == 0b1111
    }
    pub fn is_empty_walls(&self) -> bool {
        self.walls_bit & 0b1111 == 0
    }
    pub fn is_visited(&self) -> bool {
        self.walls_bit & 0b1111 != 0b1111
    }

    pub fn is_north_wall(&self) -> bool {
        self.walls_bit() & 0b1000 != 0
    }
    pub fn is_east_wall(&self) -> bool {
        self.walls_bit() & 0b0100 != 0
    }
    pub fn is_south_wall(&self) -> bool {
        self.walls_bit() & 0b0010 != 0
    }
    pub fn is_west_wall(&self) -> bool {
        self.walls_bit() & 0b0001 != 0
    }
}

impl Cell {
    pub fn update(&mut self) {}
    pub fn draw_sprite(&self, sprites: &Sprites, x_pos: &f32, y_pos: &f32, tile_size: &f32) {
        let dest_size = Some(Vec2::new(
            settings::cell::playing::SIZE,
            settings::cell::playing::SIZE,
        ));

        let rotation: Option<f32> = match self.walls_bit & 0b1111 {
            // one wall is empty
            0b0111 => Some(std::f32::consts::PI),
            0b1011 => Some(-1f32 / 2f32 * std::f32::consts::PI),
            0b1101 => Some(0f32 * std::f32::consts::PI),
            0b1110 => Some(1f32 / 2f32 * std::f32::consts::PI),
            _ => None,
        };

        if rotation.is_some() {
            draw_texture_ex(
                &sprites.walls.wall_tlr,
                *x_pos,
                *y_pos,
                WHITE,
                DrawTextureParams {
                    dest_size,
                    source: None,
                    rotation: rotation.unwrap(),
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                },
            )
        } else {
            let rotation: Option<f32> = match self.walls_bit & 0b1111 {
                0b1001 => Some(0f32),
                0b1100 => Some(-1.5f32 * std::f32::consts::PI),
                0b0110 => Some(1f32 * std::f32::consts::PI),
                0b0011 => Some(-0.5f32 * std::f32::consts::PI),
                _ => None,
            };
            if rotation.is_some() {
                draw_texture_ex(
                    &sprites.walls.wall_tl,
                    *x_pos,
                    *y_pos,
                    WHITE,
                    DrawTextureParams {
                        dest_size,
                        source: None,
                        rotation: rotation.unwrap(),
                        flip_x: false,
                        flip_y: false,
                        pivot: None,
                    },
                )
            } else {
                let rotation: Option<f32> = match self.walls_bit & 0b1111 {
                    0b1000 => Some(0f32),
                    0b0100 => Some(0.5f32 * std::f32::consts::PI),
                    0b0010 => Some(1f32 * std::f32::consts::PI),
                    0b0001 => Some(1.5f32 * std::f32::consts::PI),
                    _ => None,
                };
                if rotation.is_some() {
                    draw_texture_ex(
                        &sprites.walls.wall_t,
                        *x_pos,
                        *y_pos,
                        WHITE,
                        DrawTextureParams {
                            dest_size,
                            source: None,
                            rotation: rotation.unwrap(),
                            flip_x: false,
                            flip_y: false,
                            pivot: None,
                        },
                    )
                } else {
                    let rotation: Option<f32> = match self.walls_bit & 0b1111 {
                        0b1010 => Some(0f32),
                        0b0101 => Some(0.5f32 * std::f32::consts::PI),
                        _ => None,
                    };
                    if rotation.is_some() {
                        draw_texture_ex(
                            &sprites.walls.wall_tb,
                            *x_pos,
                            *y_pos,
                            WHITE,
                            DrawTextureParams {
                                dest_size,
                                source: None,
                                rotation: rotation.unwrap(),
                                flip_x: false,
                                flip_y: false,
                                pivot: None,
                            },
                        )
                    }
                }
            }
        }
    }

    pub fn draw_full(&self, x_pos: &f32, y_pos: &f32, tile_size: &f32, wall_size: &f32) {
        if self.walls_bit() & 0b1000 != 0 {
            draw_line(
                *x_pos,
                *y_pos,
                x_pos + tile_size,
                *y_pos,
                *wall_size,
                settings::menu::cell::COLOR,
            )
        }
        if self.walls_bit() & 0b0100 != 0 {
            draw_line(
                *x_pos + tile_size,
                *y_pos,
                x_pos + tile_size,
                y_pos + tile_size,
                *wall_size,
                settings::menu::cell::COLOR,
            )
        }

        if self.walls_bit() & 0b0010 != 0 {
            draw_line(
                *x_pos,
                y_pos + tile_size,
                x_pos + tile_size,
                y_pos + tile_size,
                *wall_size,
                settings::menu::cell::COLOR,
            )
        }

        if self.walls_bit() & 0b0001 != 0 {
            draw_line(
                *x_pos,
                *y_pos,
                *x_pos,
                y_pos + tile_size,
                *wall_size,
                settings::menu::cell::COLOR,
            )
        }
    }
    pub fn draw_reduced(&self, x_pos: &f32, y_pos: &f32, tile_size: &f32, wall_size: &f32) {
        if self.walls_bit() & 0b0100 != 0 {
            draw_line(
                *x_pos + tile_size,
                *y_pos,
                x_pos + tile_size,
                y_pos + tile_size,
                *wall_size,
                settings::menu::cell::COLOR,
            )
        }

        if self.walls_bit() & 0b0010 != 0 {
            draw_line(
                *x_pos,
                y_pos + tile_size,
                x_pos + tile_size,
                y_pos + tile_size,
                *wall_size,
                settings::menu::cell::COLOR,
            )
        }
    }
    pub fn draw_dir_to_finish(
        &self,
        x_pos: &f32,
        y_pos: &f32,
        tile_size: &f32,
        wall_size: &f32,
        sprites: &Sprites,
    ) {

        let rotation = match self.step {
            CellStep::Direction(Directions::North) => {std::f32::consts::PI *1.5f32 },
            CellStep::Direction(Directions::East) => {0f32},
            CellStep::Direction(Directions::South) => {std::f32::consts::PI /2f32 },
            CellStep::Direction(Directions::West) => {std::f32::consts::PI },
            CellStep::Finish => {std::f32::consts::PI},
            CellStep::Unvisited => {std::f32::consts::PI},
        };


        let dest_size = Vec2::new(settings::cell::playing::SIZE, settings::cell::playing::SIZE) ;
        draw_texture_ex(
            &sprites.arrow,
            *x_pos ,//+ dest_size.x,
            *y_pos,//  + dest_size.y,
            WHITE,
            DrawTextureParams {
                dest_size : Some(dest_size), // Some(dest_size),
                source: None,
                rotation: rotation,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let cell = Cell::new();
        assert_eq!(cell.walls_bit, 0b1111);
    }
}
