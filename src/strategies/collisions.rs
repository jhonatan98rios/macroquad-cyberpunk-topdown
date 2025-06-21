use crate::enemies::{Enemy, EnemyStatus};
use crate::player::Player;
use super::CollisionStrategy;

pub struct AABBCollision;

impl CollisionStrategy for AABBCollision {
    fn check_collisions(
        &self,
        enemies: &mut [Enemy],
        player: &mut Player,
    ) {
        for enemy in enemies.iter_mut() {
            if enemy.status != EnemyStatus::Live {
                continue;
            }

            let damage = 1.0; // Substitua por lógica de dano por inimigo se necessário

            let overlap = enemy.position.x < player.x + player.size &&
                          enemy.position.x + enemy.size.x > player.x &&
                          enemy.position.y < player.y + player.size &&
                          enemy.position.y + enemy.size.y > player.y;

            if overlap {
                player.take_damage(damage);
            }
        }
    }
}