use crate::cell;
use crate::cell::Cell;
use crate::cellMap::CellMap;
use macroquad::math::Vec2;
use macroquad::rand;
use std::collections::VecDeque;

pub fn gen_labyrinth(map: &mut CellMap) {
    let mut curr_vec = Vec2::new(*map.width() as f32, *map.height() as f32);
    let curr_cell = map.grid(curr_vec);

    while Some(curr_vec).is_some() {
        walk(curr_vec, map);
        curr_vec = hunt(map).unwrap();

        let mut curr_cell: Option<Cell> = que.pop_back();
        for direction in curr_cell.as_ref().unwrap().directions() {
            let new_vec = curr_vec + direction.vector();

            let new_cell = map.grid(new_vec);

            if new_cell.is_visited() {
                continue;
            }

            curr_cell.unwrap().remove_wall(*direction)
        }
    }
}

fn hunt(map: &mut CellMap) -> Option<Vec2> {
    for x in 0..*map.width() {
        for y in 0..*map.height() {
            let cell_vec = Vec2::new(x as f32, y as f32);
            let cell = map.grid(cell_vec);
            if cell.is_visited() {
                for direction in cell.directions() {
                    let new_cell_vec = cell_vec + direction.vector();
                    let new_cell = map.grid(new_cell_vec);
                    if !(new_cell.is_visited()) {
                        return Some(cell_vec);
                    }
                }
            }
        }
    }
    None // generation completed
}
fn walk(vec: Vec2, map: CellMap) {

}

fn is_vector_in_boundary(new_vec: Vec2, map: &CellMap) -> bool {
    !((new_vec.x as i32) < 0
        || (new_vec.x as i32 > *map.width() as i32)
        || (new_vec.y as i32) < 0
        || (new_vec.y as i32 > *map.height() as i32))
}

fn rand_direction() -> Vec2 {
    let num = rand::gen_range(0, 4);

    let dir = match num {
        0 => (-1, -1),
        1 => (-1, 1),
        2 => (1, 1),
        3 => (1, -1),
        _ => (0, 0),
    };
    Vec2::new(dir.0 as f32, dir.1 as f32)
}
