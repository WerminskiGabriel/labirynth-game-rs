use crate::directions::Directions;
use getset::{Getters, MutGetters};
use macroquad::rand;

#[derive(Getters, MutGetters, Clone, Debug)]
pub struct Cell {
    #[getset(get = "pub", get_mut = "pub")]
    directions: [Directions; 4],
    #[getset(get = "pub", get_mut = "pub")]
    walls_bit: u8,
    //#[getset(get="pub", get_mut="pub")]
    // is_visited : bool,
}
impl Cell {
    pub fn new() -> Self {
        Self {
            directions: Self::gen_directions(),
            walls_bit: 0b1111, //NESW
                               //is_visited: false,
        }
    }
    pub fn gen_directions() -> [Directions; 4] {
        let mut nums: [u8; 4] = [0, 1, 2, 3];
        let mut max_idx = 3;

        let mut new_directions: [Directions; 4] = [Directions::North; 4];
        let mut directions_idx = 0;

        while max_idx >= 0 {
            let rnd = rand::gen_range(0, max_idx);
            let new_direction_idx = nums[rnd];
            nums[rnd] = nums[max_idx];
            max_idx -= 1;
            new_directions[directions_idx] = match new_direction_idx {
                0 => Directions::North,
                1 => Directions::East,
                2 => Directions::South,
                3 => Directions::West,
                _ => Directions::North, // Should not occur
            }
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
