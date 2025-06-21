use super::MovementStrategy;
use macroquad::prelude::*;
use crate::constants::{WORLD_HEIGHT, WORLD_WIDTH};
use crate::enemies::{Enemy, EnemyInfo, EnemyStatus};

pub struct BoidsMovement {
    pub visual_range: f32,
    pub separation_dist: f32,
    pub max_speed: f32,
    pub player_weight: f32,
    pub player_distance: f32,
    pub noise_strength: f32,
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
}

impl MovementStrategy for BoidsMovement {
    fn move_enemy(
        &self,
        enemy: &mut Enemy,
        target: Vec2,
        _time: f32,
        index: usize,
        all_enemies: &[EnemyInfo],
    ) {
        let mut separation = Vec2::ZERO;
        let mut alignment = Vec2::ZERO;
        let mut cohesion = Vec2::ZERO;
        let mut neighbors = 0;

        for (i, other) in all_enemies.iter().enumerate() {
            if i == index || other.status != EnemyStatus::Live {
                continue;
            }

            let dist = enemy.position.distance(other.position);

            if dist < self.visual_range {
                if dist < self.separation_dist {
                    let separation_force = (1.0 - (dist / self.separation_dist)).powf(2.0);
                    separation += (enemy.position - other.position).normalize() * separation_force;
                }

                alignment += (other.position - enemy.position).normalize_or_zero();
                cohesion += other.position;
                neighbors += 1;
            }
        }

        let mut velocity = Vec2::ZERO;

        if neighbors > 0 {
            let n = neighbors as f32;

            separation = separation.normalize_or_zero() * self.separation_weight;
            alignment = (alignment / n).normalize_or_zero() * self.alignment_weight;
            cohesion = ((cohesion / n) - enemy.position).normalize_or_zero() * self.cohesion_weight;

            velocity += separation + alignment + cohesion;
        }

        let to_player = target - enemy.position;
        let player_dist = to_player.length();
        let player_influence = 1.0 - (player_dist / self.player_distance).min(1.0).max(0.0);

        velocity += to_player.normalize_or_zero() * self.player_weight * player_influence.powf(0.5);

        velocity += Vec2::new(
            rand::gen_range(-1.0, 1.0),
            rand::gen_range(-1.0, 1.0),
        ) * self.noise_strength;

        velocity = velocity.normalize_or_zero() * self.max_speed;
        enemy.position += velocity;

        // Atualiza o movimento
        if velocity.length_squared() > 0.0 {
            enemy.last_movement = velocity.normalize();
        }

        enemy.position.x = enemy.position.x.clamp(0.0, WORLD_WIDTH);
        enemy.position.y = enemy.position.y.clamp(0.0, WORLD_HEIGHT);
    }
}