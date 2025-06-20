use macroquad::prelude::*;
use crate::render::Renderable;

use super::building::Building;

pub struct BuildingsManager {
    pub buildings: Vec<Building>,
}

impl BuildingsManager {
    pub fn new() -> Self {
        BuildingsManager {
            buildings: Vec::new(),
        }
    }

    pub fn add(&mut self, building: Building) {
        self.buildings.push(building);
    }

    // Get the player position and draw only the buildings that have y position greater than the player y position
    pub fn draw_above_player(&self, player_y: f32) {
        for building in &self.buildings {
            if building.position.y > player_y {
                building.draw();
            }
        }
    }

    pub fn draw_below_player(&self, player_y: f32) {
        for building in &self.buildings {
            if building.position.y <= player_y {
                building.draw();
            }
        }
    }

    pub fn get_buildings(&self) -> &Vec<Building> {
        &self.buildings
    }
}