use macroquad::prelude::*;
use crate::constants::{WORLD_WIDTH, WORLD_HEIGHT};

pub struct FloorTileSet {
    texture: Texture2D,
    tile_size: Vec2,
    tiles_per_row: u8,
}

pub enum FloorTile {
    Sidewalk = 0,
    Top = 1,
    Right = 2,
    Left = 3,
    Bottom = 4,
    Asphalt = 5,
    Manhole = 6,
    BottomLeft = 7,
    BottomRight = 8,
    TopLeft = 9,
    TopRight = 10,
}

impl FloorTileSet {
    pub async fn load(path: &str) -> Self {
        // let texture = load_texture(path).await.unwrap();
        // texture.set_filter(FilterMode::Nearest); // opcional: manter pixel-art
        let texture = match load_texture(path).await {
            Ok(t) => {
                t.set_filter(FilterMode::Nearest); // opcional: manter pixel-art
                Some(t)
            },
            Err(_) => {
                println!("Failed to load player texture, falling back to rectangle");
                None
            }
        };

        FloorTileSet {
            texture: texture.unwrap(),
            tile_size: vec2(64.0, 64.0),
            tiles_per_row: 11,
        }
    }

    pub fn draw_tile(&self, tile_index: u8, position: Vec2) {
        let src_x = (tile_index % self.tiles_per_row) as f32 * self.tile_size.x;
        let src_y = 0.0; // todos os tiles estão na primeira linha
        draw_texture_ex(
            &self.texture,
            position.x,
            position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.tile_size),
                flip_y: true,
                source: Some(Rect::new(
                    src_x,
                    src_y,
                    self.tile_size.x,
                    self.tile_size.y,
                )),
                ..Default::default()
            },
        );
    }

    pub fn draw_floor(&self) {
        let tile_size = self.tile_size;
        let cols = (WORLD_WIDTH / tile_size.x).ceil() as i32;
        let rows = (WORLD_HEIGHT / tile_size.y).ceil() as i32;

        for y in 0..rows {
            for x in 0..cols {

                let mut tile_index = 5; // tile padrão

                if x == 0 && y == 0 {
                    tile_index = FloorTile::BottomLeft as u8; // Bottom Left 
                } else 
                if x == cols - 1 && y == rows - 1 {
                    tile_index = FloorTile::TopRight as u8; // Top Right
                } else 
                if x == cols - 1 && y == 0 {
                    tile_index = FloorTile::BottomRight as u8; //
                } else 
                if x == 0 && y == rows - 1 {
                    tile_index = FloorTile::TopLeft as u8; //
                } else 
                
                
                if y == 0 {
                    tile_index = FloorTile::Bottom as u8; // linha superior
                } else if y == rows - 1 {
                    tile_index = FloorTile::Top as u8; // linha inferior
                } else if x == 0 {
                    tile_index = FloorTile::Left as u8; // coluna esquerda
                } else if x == cols - 1 {
                    tile_index = FloorTile::Right as u8; // coluna direita
                }

                let pos = vec2(x as f32 * tile_size.x, y as f32 * tile_size.y);
                self.draw_tile(tile_index, pos);
            }
        }
    }
}
