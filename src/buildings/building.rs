use macroquad::prelude::*;
use crate::{render::Renderable};

pub struct Building {
    pub texture: Texture2D,
    pub position: Vec2,
    pub size: Vec2,
    pub collision_rect: Rect,
}

impl Building {
    pub async fn new(texture_path: &str, position: Vec2, size: Vec2, collision_position: Vec2, collision_size: Vec2) -> Self {
        let texture = load_texture(texture_path).await.unwrap();
        let collision_rect = Rect::new(collision_position.x, collision_position.y, collision_size.x, collision_size.y);

        Building {
            texture,
            position,
            size,
            collision_rect,
        }
    }

    pub fn bounds(&self) -> Rect {
        self.collision_rect
    }
}

impl Renderable for Building {
    fn position(&self) -> Vec2 {
        return self.position;
    }

    fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.size),
                flip_y: true,
                ..Default::default()
            },
        );
    }
}