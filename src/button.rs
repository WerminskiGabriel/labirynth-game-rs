use crate::settings;
use macroquad::prelude::*;

pub fn draw_button(vec: Vec2, w: f32, h: f32, text: &str, font: Font) -> bool {
    let mouse_pos = mouse_position();
    let mut button_color = settings::menu::button::COLOR;
    let mut text_color = settings::menu::button::FONT_COLOR;

    let is_hover = mouse_pos.0 >= vec.x
        && mouse_pos.0 <= vec.x + w
        && mouse_pos.1 >= vec.y
        && mouse_pos.1 <= vec.y + h;

    if is_hover {
        button_color = settings::menu::button::COLOR_HOVER;
        text_color = settings::menu::button::FONT_COLOR_HOVER;
    }

    let dims = measure_text(
        text,
        Some(&font),
        settings::menu::button::FONT_SIZE as u16,
        1f32,
    );
    let center_x = vec.x + w / 2f32;
    let center_y = vec.y + h / 2f32;
    let x_pos = center_x - dims.width / 2f32;
    let y_pos = center_y + dims.offset_y / 2f32;

    draw_rectangle(vec.x, vec.y, w, h, button_color);
    draw_rectangle_lines(vec.x, vec.y, w, h, settings::menu::cell::THICKNESS * 2f32, settings::menu::button::COLOR_LINES);
    draw_text_ex(
        text,
        x_pos,
        y_pos,
        TextParams {
            font: Some(&font),
            font_size: settings::menu::button::FONT_SIZE as u16,
            color: text_color,
            ..Default::default()
        },
    );

    is_hover && is_mouse_button_pressed(MouseButton::Left)
}
