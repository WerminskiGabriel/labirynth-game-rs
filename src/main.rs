use macroquad::prelude::*;

#[macroquad::main("labirynth")]
async fn main() {

    loop {

        draw_circle( screen_width()/2f32, screen_height()/2f32, 10f32, WHITE );
        next_frame().await
    }
}
