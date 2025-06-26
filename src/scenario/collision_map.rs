use macroquad::prelude::*;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct CollisionMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<u8>,
}

impl CollisionMap {
    pub async fn load_from_csv(path: &str) -> Self {
        let csv_data = load_string(path).await.unwrap();

        let mut tiles = vec![];
        let mut width = 0;
        let mut height = 0;

        for line in csv_data.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let row: Vec<u8> = line
                .split(',')
                .map(|s| u8::from_str(s.trim()).unwrap())
                .collect();

            width = row.len();
            height += 1;
            tiles.extend(row);
        }

        CollisionMap { width, height, tiles }
    }

    pub fn is_colliding(&self, position: Vec2) -> bool {
        let tile_size = 64.0;
        let x = (position.x / tile_size) as usize;
        let y = (position.y / tile_size) as usize;

        if x >= self.width || y >= self.height {
            return false;
        }

        self.tiles[y * self.width + x] != 0
    }
}
