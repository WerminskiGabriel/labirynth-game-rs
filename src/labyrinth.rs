use crate::cell;
use crate::cell::Cell;
use crate::cellMap::CellMap;
use macroquad::math::Vec2;
use macroquad::rand;
use std::collections::VecDeque;

pub fn gen_labyrinth(map: &mut CellMap) {
    let mut curr_vec = Vec2::new(0f32, 0f32);

    walk(curr_vec, map);
    while let Some(curr_vec) = hunt(map) {
        walk(curr_vec, map);
    }
    fn hunt(map: &mut CellMap) -> Option<Vec2> {
        for x in 0..*map.width() {
            for y in 0..*map.height() {
                let cell_vec = Vec2::new(x as f32, y as f32);
                let cell = map.grid(cell_vec);
                if cell.is_visited() {
                    for direction in Cell::gen_directions() {
                        let new_cell_vec = cell_vec + direction.vector();
                        if is_vector_in_boundary(new_cell_vec, &map) {
                            let new_cell = map.grid(new_cell_vec);
                            if !(new_cell.is_visited()) {
                                return Some(cell_vec);
                            }
                        }
                    }
                }
            }
        }
        None // generation completed
    }
    fn walk(vec: Vec2, map: &mut CellMap) {
        let mut curr_vec = vec;
        loop {
            let mut moved: bool = false;

            let directions = Cell::gen_directions();
            for direction in directions {
                let new_vec = curr_vec + direction.vector();
                if is_vector_in_boundary(new_vec, map) && !(map.grid_mut(new_vec).is_visited()) {
                    map.grid_mut(curr_vec).remove_wall(direction);

                    map.grid_mut(new_vec).remove_wall(direction.opposite());
                    curr_vec = new_vec;

                    moved = true;
                    break;
                }
            }

            if !moved {
                break;
            }
        }
    }
}

fn is_vector_in_boundary(new_vec: Vec2, map: &CellMap) -> bool {
    !((new_vec.x as i32) < 0
        || (new_vec.x as i32 >= *map.width() as i32)
        || (new_vec.y as i32) < 0
        || (new_vec.y as i32 >= *map.height() as i32))
}
