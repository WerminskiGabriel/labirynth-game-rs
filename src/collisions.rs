use crate::cellMap::CellMap;
use crate::directions::Directions;
use crate::settings;
use macroquad::math::Vec2;

pub fn walls_collision(
    obj_pos: &Vec2,
    obj_radius: f32,
    tmp_pos: &mut f32,
    map: &CellMap,
    direction: Directions,
    offset: f32 ,
) -> bool {
    let wall_size = settings::cell::playing::THICKNESS;
    let cell_size = settings::cell::playing::SIZE;

    let start_x_idx: f32 = (obj_pos.x / cell_size).floor();
    let start_y_idx = (obj_pos.y / cell_size).floor();

    let vec = Vec2::new(start_x_idx, start_y_idx);
    let cell = map.grid(vec);

    match direction {
        Directions::North => {
            if cell.is_north_wall()
                && ((*tmp_pos - obj_radius - wall_size + offset) / cell_size).floor() != start_y_idx
            {
                *tmp_pos = start_y_idx * cell_size + wall_size + obj_radius;
                return true;
            }
        }
        Directions::East => {
            if cell.is_east_wall()
                && ((*tmp_pos + obj_radius + wall_size + offset) / cell_size).floor() != start_x_idx
            {
                *tmp_pos = (start_x_idx + 1f32) * cell_size - wall_size - obj_radius;
                return true;
            }
        }
        Directions::South => {
            if cell.is_south_wall()
                && ((*tmp_pos + obj_radius + wall_size + offset) / cell_size).floor() != start_y_idx
            {
                *tmp_pos = (start_y_idx + 1f32) * cell_size - wall_size - obj_radius;
                return true;
            }
        }
        Directions::West => {
            if cell.is_west_wall()
                && ((*tmp_pos - obj_radius - wall_size - offset) / cell_size).floor() != start_x_idx
            {
                *tmp_pos = start_x_idx * cell_size + wall_size + obj_radius;
                return true;
            }
        }
    }
    false
}
