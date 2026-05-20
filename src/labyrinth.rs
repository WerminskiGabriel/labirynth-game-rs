use crate::CellStep::CellStep;
use crate::cell;
use crate::cell::Cell;
use crate::cellMap::CellMap;
use crate::directions::Directions;
use macroquad::math::Vec2;
use macroquad::miniquad::start;
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

pub fn find_finish(map: &CellMap, start_pos: Vec2) -> Vec2 {
    let map_width = map.width();
    let mut visited = vec![false; map.width() * map.height()];

    let mut que = VecDeque::new();
    que.push_back(start_pos);

    let mut curr_pos = start_pos;
    while !que.is_empty() {
        curr_pos = que.pop_front().unwrap();
        let curr_cell = map.grid(curr_pos);

        let directions = [
            (Directions::North, curr_cell.is_north_wall()),
            (Directions::East, curr_cell.is_east_wall()),
            (Directions::South, curr_cell.is_south_wall()),
            (Directions::West, curr_cell.is_west_wall()),
        ];

        for (dir, is_wall) in directions {
            if !is_wall {
                let new_pos = curr_pos + dir.vector();
                if check_visit_grid(&new_pos, &visited, map_width) {
                    visit_grid(&new_pos, &mut visited, map_width);
                    que.push_back(new_pos);
                }
            }
        }
    }
    return curr_pos;
}

pub fn fill_path_to_finish(map: &mut CellMap, start_pos: Vec2) -> Vec2 {
    let finish_pos = find_finish(map, start_pos);
    let mut finish_cell  = map.grid_mut(finish_pos);
    *finish_cell.step_mut() = CellStep::Finish;

    let map_width = map.width();
    let mut visited = vec![false; map.width() * map.height()];

    let mut que = VecDeque::new();
    que.push_back(finish_pos);

    while !que.is_empty() {
        let curr_pos = que.pop_front().unwrap();
        let curr_cell = map.grid(curr_pos);

        let directions = [
            (Directions::North, curr_cell.is_north_wall()),
            (Directions::East, curr_cell.is_east_wall()),
            (Directions::South, curr_cell.is_south_wall()),
            (Directions::West, curr_cell.is_west_wall()),
        ];

        for (dir, is_wall) in directions {
            if !is_wall {
                let new_pos = curr_pos + dir.vector();
                let new_cell = map.grid_mut(new_pos);
                if *new_cell.step() == CellStep::Unvisited {
                    *new_cell.step_mut() = CellStep::Direction(dir.opposite());
                    que.push_back(new_pos);
                }
            }
        }
    }
    return finish_pos;
}

fn check_visit_grid(vec: &Vec2, visited: &Vec<bool>, width: &usize) -> bool {
    visited[vec.y as usize * width + vec.x as usize]
}
fn visit_grid(vec: &Vec2, visited: &mut Vec<bool>, width: &usize) {
    visited[vec.y as usize * width + vec.x as usize] = true;
}
