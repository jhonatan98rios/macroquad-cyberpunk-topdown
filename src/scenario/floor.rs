use macroquad::prelude::*;
use super::floor_map::TileMap;

pub struct FloorTileSet {
    texture: Texture2D,
    tile_size: Vec2,
    tiles_per_row: u8,
}


impl FloorTileSet {
    pub async fn load(path: &str) -> Self {
        let texture = load_texture(path).await.unwrap();

        FloorTileSet {
            texture: texture,
            tile_size: vec2(64.0, 64.0),
            tiles_per_row: 15,
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

    pub fn draw_tilemap(&self, map: &TileMap) {
        for y in 0..map.height {
            for x in 0..map.width {
                let index = y * map.width + x;
                let tile_id = map.tiles[index];

                // Flip Y here:
                let draw_y = (map.height - 1 - y) as f32;

                let position = vec2(
                    x as f32 * self.tile_size.x,
                    draw_y * self.tile_size.y,
                );

                self.draw_tile(tile_id, position);
            }
        }
    }
}
