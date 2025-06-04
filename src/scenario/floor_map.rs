use macroquad::prelude::*;
use std::str::FromStr;

pub struct TileMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<u8>,
}

impl TileMap {
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

        TileMap { width, height, tiles }
    }
}
