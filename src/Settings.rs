use macroquad::prelude::*;

const COLOR_A: Color = Color {
    r: 128f32 / 255.0,
    g: 124f32 / 255.0,
    b: 121f32 / 255.0,
    a: 1f32,
};
const COLOR_B: Color = Color {
    r: 122f32 / 255.0,
    g: 111f32 / 255.0,
    b: 111f32 / 255.0,
    a: 1f32,
};
const COLOR_C: Color = Color {
    r: 83f32 / 255.0,
    g: 90f32 / 255.0,
    b: 99f32 / 255.0,
    a: 1f32,
};
const COLOR_D: Color = Color {
    r: 57f32 / 255.0,
    g: 574f32 / 255.0,
    b: 75f32 / 255.0,
    a: 1f32,
};
const COLOR_E: Color = Color {
    r: 29f32 / 255.0,
    g: 29f32 / 255.0,
    b: 46f32 / 255.0,
    a: 1f32,
};
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
    pub const SIZE: f32 = 40f32;

    pub const WIDTH: f32 = SIZE / 2f32;
    pub const HEIGHT: f32 = SIZE * 2f32;
    pub const COLOR: Color = COLOR_B;
    pub const HP: f32 = 2000f32;
}

pub mod enemy {
    use super::*;

    pub const MAX_COUNT: usize = 1000usize;
    pub mod ghost {
        use super::*;

        pub const SPEED: f32 = 2f32;
        pub const HP: f32 = 200f32;
        pub const DMG_RADIUS: f32 = 100f32;
        pub const DMG: f32 = 2f32;
        pub const SIZE: f32 = 30f32;
        pub const COOLDOWN: f32 = 2f32;
    }
    pub mod goblin {
        use super::*;

        pub const SPEED: f32 = 5f32;
        pub const HP: f32 = 200f32;
        pub const DMG_RADIUS: f32 = 100f32;
        pub const DMG: f32 = 2f32;
        pub const SIZE: f32 = 30f32;
        pub const COOLDOWN: f32 = 2f32;
    }
}
pub mod window {
    use super::*;
    pub const TITLE: &str = "Maze";
    pub const WIDTH: i32 = 1440;
    pub const HEIGHT: i32 = 900;
}

pub mod bullet {
    use super::*;

    pub const LIFETIME: f32 = 10f32;
    pub const SIZE: f32 = player::SIZE * 0.5f32;
    pub const COLOR: Color = COLOR_B;
    pub const SPEED: f32 = 2f32 * SIZE;
}
pub mod cell {
    use super::*;
    pub mod playing {

        use super::*;
        use super::*;
        use crate::settings::COLOR_RED;
        use macroquad::color::Color;
        pub const SIZE: f32 = 320f32;
        pub const THICKNESS: f32 = 30f32;
        pub const COLOR: Color = COLOR_D;
        pub const DMG: f32 = 2f32;
    }
}

pub mod map {
    use super::*;
    pub mod playing {
        use super::*;
        pub const COLS: usize = 101usize;
        pub const ROWS: usize = 101usize;
        pub const START_POS: Vec2 = Vec2::new(50f32, 50f32);
    }
    pub mod menu {
        use super::*;
        pub const COLS: usize = 60usize;
        pub const ROWS: usize = 100usize;
        pub const START_POS: Vec2 = Vec2::new(50f32, 50f32);
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
        pub const COLOR: Color = COLOR_D;
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

        pub const COLOR: Color = COLOR_A;
        pub const COLOR_HOVER: Color = COLOR_B;
        pub const COLOR_LINES: Color = COLOR_E;
    }
}

pub mod ui {
    use super::*;
    pub const FONT_SIZE: u16 = 30u16;
    pub const TEXT_COLOR: Color = COLOR_A;
    pub const BACKGROUND_COLOR: Color = COLOR_E;
}

pub mod button {
    use super::*;

    pub const COLOR: Color = COLOR_BLACK;
    pub const COLOR_HOVER: Color = COLOR_RED;
}
