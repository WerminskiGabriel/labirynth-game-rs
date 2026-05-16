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
const COLOR_RED2: Color = Color {
    r: 200f32 / 255.0,
    g: 0f32 / 255.0,
    b: 47f32 / 255.0,
    a: 1f32,
};
const COLOR_BLACK2: Color = Color {
    r: 47f32 / 255.0,
    g: 1f32 / 255.0,
    b: 1f32 / 255.0,
    a: 1f32,
};

const COLOR_BEIGE: Color = Color {
    r: 230f32 / 255.0,
    g: 223f32 / 255.0,
    b: 194f32 / 255.0,
    a: 1f32,
};

pub mod player {
    use super::*;

    pub const SPEED: f32 = 12f32;
    pub const SPEED_FAST: f32 = 22f32;
    pub const SIZE: f32 = 30f32;
    pub const COLOR: Color = COLOR_BEIGE;
}
pub mod window {
    use super::*;
    pub const TITLE: &str = "Maze";
    pub const WIDTH: i32 = 1440;
    pub const HEIGHT: i32 = 900;
}

pub mod playing {
    use super::*;
    pub mod cell {
        use super::*;
        pub const SIZE: f32 = 200f32;
        pub const THICKNESS: f32 = 15f32;
        pub const COLOR: Color = COLOR_RED;
    }
}
pub mod menu {
    use super::*;
    pub mod map {
        use super::*;
        pub const COLS: usize = 60usize;
        pub const ROWS: usize = 100usize;
        pub const WIDTH: f32 = 1340f32;
        pub const START_POS: Vec2 = Vec2::new(50f32, 50f32);
    }

    pub mod cell {
        use super::*;
        use crate::settings;
        pub const COLOR: Color = COLOR_RED;
        pub const THICKNESS: f32 = (SIZE * 0.6);
        pub const SIZE: f32 = map::WIDTH / settings::menu::map::ROWS as f32;
    }

    pub mod button {
        use super::*;

        pub const W: f32 = 600f32;
        pub const H: f32 = 150f32;
        pub const FONT_SIZE: f32 = 65f32;
        pub const FONT_COLOR: Color = COLOR_RED;
        pub const FONT_COLOR_HOVER: Color = COLOR_BLACK;

        pub const COLOR: Color = COLOR_BLACK;
        pub const COLOR_HOVER: Color = COLOR_RED;
        pub const COLOR_LINES: Color = COLOR_BLACK2;
    }
}

pub mod ui {
    use super::*;
    pub const FONT_SIZE: u16 = 30u16;
    pub const TEXT_COLOR: Color = COLOR_RED;
    pub const BACKGROUND_COLOR: Color = COLOR_BLACK;
}

pub mod button {
    use super::*;

    pub const COLOR: Color = COLOR_BLACK;
    pub const COLOR_HOVER: Color = COLOR_RED;
}
