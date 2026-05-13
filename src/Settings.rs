use macroquad::prelude::*;

const COLOR_RED: Color = Color {
    r: 219f32 / 255.0,
    g: 61f32 / 255.0,
    b: 62f32 / 255.0,
    a: 1f32,
};
const COLOR_BLACK: Color = Color {
    r: 63f32 / 255.0,
    g: 63f32 / 255.0,
    b: 63f32 / 255.0,
    a: 1f32,
};
const COLOR_BEIGE: Color = Color {
    r: 230f32 / 255.0,
    g: 223f32 / 255.0,
    b: 194f32 / 255.0,
    a: 1f32,
};

pub mod player {
    pub const SPEED: f32 = 20f32;
    pub const SIZE : f32 = 20f32;
}

pub mod map {
    pub const SIZE : usize = 15usize;
    pub const WIDTH : f32 = 600f32;
}

pub mod cell {
    use super::*;
    pub const COLOR : Color = COLOR_RED;
    pub const SIZE : f32 = 50f32;
}

pub mod ui {
    use super::*;
    pub const FONT_SIZE: u16 = 30u16;
    pub const TEXT_COLOR: Color = COLOR_RED;
    pub const BACKGROUND_COLOR: Color = COLOR_BLACK;
}