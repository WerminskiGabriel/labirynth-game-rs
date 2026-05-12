use macroquad::math::Vec2;

#[derive(Clone, Copy, Debug)]
pub enum Directions {
    North,
    East,
    South,
    West,
}

impl Directions {
    pub fn vector(&self) -> Vec2 {
        match self {
            Directions::North => Vec2::new(0f32, 1f32),
            Directions::East => Vec2::new(1f32, 0f32),
            Directions::South => Vec2::new(0f32, -1f32),
            Directions::West => Vec2::new(-1f32, 0f32),
        }
    }
}
