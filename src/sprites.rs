use crate::settings;
use macroquad::prelude::*;

pub struct Sprites {
    pub walls: Walls,
    pub arrow: Texture2D,
    pub player: Texture2D,
    pub enemies: Enemies,
    pub blood_frames: Vec<Texture2D>,
}
pub struct Walls {
    pub wall_tl: Texture2D,
    pub wall_t: Texture2D,
    pub wall_tlr: Texture2D,
    pub wall_tb: Texture2D,
}
pub struct Enemies {
    pub ghost: Texture2D,
    pub goblin: Texture2D,
}

impl Sprites {
    pub async fn load() -> Self {
        let sprites_path = "media/sprites/";
        let mut blood_frames = vec![];

        let max_idx = settings::death::MAX_FRAMES;
        for frame_idx in 0..=max_idx {
            let path = format!("media/sprites/blood/1_{}.png", { frame_idx });
            blood_frames.push(load_texture(&path).await.unwrap());
        }

        Self {
            walls: Walls {
                wall_t: load_texture(format!("{}cell_wall/top.png", sprites_path,).as_str())
                    .await
                    .unwrap(),
                wall_tl: load_texture(format!("{}cell_wall/top-left.png", sprites_path,).as_str())
                    .await
                    .unwrap(),
                wall_tlr: load_texture(
                    format!("{}cell_wall/top-left-right.png", sprites_path,).as_str(),
                )
                .await
                .unwrap(),
                wall_tb: load_texture(format!("{}cell_wall/top-bot.png", sprites_path,).as_str())
                    .await
                    .unwrap(),
            },
            arrow: load_texture("media/sprites/other/arrow.png").await.unwrap(),
            player: load_texture("media/sprites/player/1.png").await.unwrap(),
            enemies: Enemies {
                ghost: load_texture("media/sprites/enemy/ghost-cutie-3.png")
                    .await
                    .unwrap(),
                goblin: load_texture("media/sprites/enemy/goblin.png")
                    .await
                    .unwrap(),
            },
            blood_frames,
        }
    }
}
