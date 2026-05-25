use macroquad::prelude::*;

pub struct Sprites {
    pub wall_tl: Texture2D,
    pub wall_t: Texture2D,
    pub wall_tlr: Texture2D,
    pub wall_tb: Texture2D
}

impl Sprites {
    pub async fn load() -> Self {
        let sprites_path = "media/sprites/";

        Self {
            wall_t: load_texture(format!("{}cell_wall/top.png", sprites_path, ).as_str() ).await.unwrap(),
            wall_tl: load_texture( format!("{}cell_wall/top-left.png",sprites_path,).as_str() ).await.unwrap(),
            wall_tlr: load_texture( format!("{}cell_wall/top-left-right.png",sprites_path,).as_str()).await.unwrap(),
            wall_tb: load_texture( format!("{}cell_wall/top-bot.png",sprites_path,).as_str()).await.unwrap(),
        }

    }
}