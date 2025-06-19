    
use macroquad::prelude::*;
use super::building::Building;
use super::buildings_manager::BuildingsManager;

pub struct BuildingsFactory;

impl BuildingsFactory {
    pub async fn create_default_buildings() -> BuildingsManager {
        let mut manager = BuildingsManager::new();

        let big_building_1 = Building::new(
            "images/buildings/big_building_1.png",
            vec2(260.0, 280.0), // Posição do prédio
            vec2(512.0, 950.0), // Tamanho do prédio
            vec2(280.0, 380.0), // Posição da area de colisão (pos.x + 20, pos.y + 60)
            vec2(472.0, 450.0), // Tamanho da area de colisão (size.x - 40, Y size depends on the building size)
        ).await;
        manager.add(big_building_1);

        let book_store_1 = Building::new(
            "images/buildings/book_store_1.png",
            vec2(1160.0, 440.0), // Posição do prédio
            vec2(512.0, 740.0), // Tamanho do prédio
            vec2(1180.0, 480.0), // Posição da area de colisão (pos.x + 20, pos.y + 60)
            vec2(472.0, 480.0), // Tamanho da area de colisão (size.x - 40, Y size depends on the building size)
        ).await;
        manager.add(book_store_1);

        let hotel_1 = Building::new(
            "images/buildings/hotel_1.png",
            vec2(260.0, 260.0), // Posição do prédio
            vec2(256.0, 440.0), // Tamanho do prédio
            vec2(280.0, 320.0), // Posição da area de colisão (pos.x + 20, pos.y + 60)
            vec2(216.0, 80.0), // Tamanho da area de colisão (size.x - 40, Y size depends on the building size)
        ).await;
        manager.add(hotel_1);

        let shopping_1 = Building::new(
            "images/buildings/shopping_1.png",
            vec2(510.0, 260.0), // Posição do prédio
            vec2(256.0, 500.0), // Tamanho do prédio
            vec2(530.0, 320.0), // Posição da area de colisão (pos.x + 20, pos.y + 60)
            vec2(216.0, 100.0), // Tamanho da area de colisão (size.x - 40, Y size depends on the building size)
        ).await;
        manager.add(shopping_1);

        let japan_store_1 = Building::new(
            "images/buildings/japan_store_1.png",
            vec2(1160.0, 270.0), // Posição do prédio
            vec2(254.0, 550.0), // Tamanho do prédio
            vec2(1180.0, 320.0), // Posição da area de colisão (pos.x + 20, pos.y + 60)
            vec2(214.0, 100.0), // Tamanho da area de colisão (size.x - 40, Y size depends on the building size)
        ).await;
        manager.add(japan_store_1);

        let cafe_1 = Building::new(
            "images/buildings/cafe_1.png",
            vec2(1410.0, 260.0), // Posição do prédio
            vec2(250.0, 450.0), // Tamanho do prédio
            vec2(1430.0, 320.0), // Posição da area de colisão (pos.x + 20, pos.y + 60)
            vec2(210.0, 100.0), // Tamanho da area de colisão (size.x - 40, Y size depends on the building size)
        ).await;
        manager.add(cafe_1);

        

        manager
    }
}
