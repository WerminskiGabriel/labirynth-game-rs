use macroquad::prelude::*;
use crate::enemy::EnemyType;
use crate::settings;
use crate::sprites::Sprites;

pub struct Death {
    pub position: Vec2,
    pub animation_frame: usize,
    pub timer: f32,
}

impl Death {
    pub fn update( &mut self, ft : f32) -> bool {
        let max_animation_frame = settings::death::MAX_FRAMES;
        let frame_dur = settings::death::FRAME_DURATION;

        self.timer -= ft ;
        if self.timer <= 0f32 {
            self.timer = frame_dur;
            self.animation_frame += 1;
            return true;
        }

        return false;
    }
    pub fn new( pos : &Vec2) -> Self {
        Self{
            position : *pos,
            animation_frame : 0,
            timer : settings::death::FRAME_DURATION,
        }
    }

    pub fn draw(&self, player_pos: &Vec2, sprite: &Sprites){
        let window_w = settings::window::WIDTH as f32;
        let window_h = settings::window::HEIGHT as f32;
        let offset = 10f32;

        let dest_size =  Vec2::new(settings::death::SIZE,settings::death::SIZE);

        let bullet_camera_x = self.position.x - player_pos.x + window_w / 2f32;
        let bullet_camera_y = self.position.y - player_pos.y + window_h / 2f32;

        let curr_sprite = sprite.blood_frames[self.animation_frame].clone();

        if -offset <= bullet_camera_x
            && bullet_camera_x - window_w <= offset
            && -offset <= bullet_camera_y
            && bullet_camera_y - window_h <= offset
        {
            draw_texture_ex(
                &curr_sprite,
                bullet_camera_x - dest_size.x / 2f32,
                bullet_camera_y - dest_size.y / 2f32,

                WHITE,

                DrawTextureParams {
                    dest_size: Some( dest_size ),
                    source: None,
                    rotation: 0f32,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                },
            );
        }
    }
}