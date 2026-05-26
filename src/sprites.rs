use macroquad::prelude::*;

pub struct Sprites {
    pub walls: Walls,
    pub arrow: Texture2D,
    pub player: Texture2D,
}
pub struct Walls {
    pub wall_tl: Texture2D,
    pub wall_t: Texture2D,
    pub wall_tlr: Texture2D,
    pub wall_tb: Texture2D,
}

impl Sprites {
    pub async fn load() -> Self {
        let sprites_path = "media/sprites/";

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
        }
    }
}
