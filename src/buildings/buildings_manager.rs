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

    pub fn get_buildings(&self) -> &Vec<Building> {
        &self.buildings
    }
}