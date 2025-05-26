use macroquad::prelude::*;
use crate::enemies::{EnemyStatus, EnemyData};
use crate::player::Player;
use super::CollisionStrategy;

pub struct AABBCollision;

impl CollisionStrategy for AABBCollision {
    fn check_collisions(
        &mut self,
        positions: &mut Vec<Vec2>,
        sizes: &Vec<Vec2>,
        data: &mut Vec<EnemyData>,
        player: &mut Player
    ) {
        for i in 0..positions.len() {
            if data[i].status != EnemyStatus::Live {
                continue;
            }

            let enemy_pos = positions[i];
            let enemy_size = sizes[i];
            let damage = 1.0; // Replace with the damage[i]

            let overlap = enemy_pos.x < player.x + player.size &&
                          enemy_pos.x + enemy_size.x > player.x &&
                          enemy_pos.y < player.y + player.size &&
                          enemy_pos.y + enemy_size.y > player.y;

            if overlap {
                player.take_damage(damage);
            }
        }
    }
}
