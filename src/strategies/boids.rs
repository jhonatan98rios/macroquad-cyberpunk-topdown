use super::MovementStrategy;
use macroquad::prelude::*;
use crate::constants::{WORLD_WIDTH, WORLD_HEIGHT};

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
        position: &mut Vec2,
        target: Vec2,
        _time: f32,
        index: usize,
        all_positions: &[Vec2],
    ) {
        let mut separation = Vec2::ZERO;
        let mut alignment = Vec2::ZERO;
        let mut cohesion = Vec2::ZERO;
        let mut neighbors = 0;

        for (i, &other_pos) in all_positions.iter().enumerate() {
            if i == index { continue; }

            let dist = position.distance(other_pos);

            if dist < self.visual_range {
                // Separation: steer to avoid crowding
                if dist < self.separation_dist {
                    let separation_force = (1.0 - (dist / self.separation_dist)).powf(2.0);
                    separation += (*position - other_pos).normalize() * separation_force;
                }

                // Alignment: steer towards average heading
                alignment += (other_pos - *position).normalize_or_zero();

                // Cohesion: steer towards average position
                cohesion += other_pos;
                
                neighbors += 1;
            }
        }

        let mut velocity = Vec2::ZERO;

        if neighbors > 0 {
            let n = neighbors as f32;
            separation = separation.normalize_or_zero() * self.separation_weight;
            alignment = (alignment / n).normalize_or_zero() * self.alignment_weight;
            cohesion = ((cohesion / n) - *position).normalize_or_zero() * self.cohesion_weight;

            velocity += separation + alignment + cohesion;
        }

        // Player attraction/repulsion
        let to_player = target - *position;
        // With this more gradual influence:
        let player_dist = to_player.length();
        let player_influence = 1.0 - (player_dist / self.player_distance).min(1.0).max(0.0);
        velocity += to_player.normalize_or_zero() * 
            self.player_weight * 
            player_influence.powf(0.5); // More gradual falloff

        // Add some randomness
        velocity += Vec2::new(
            rand::gen_range(-1.0, 1.0),
            rand::gen_range(-1.0, 1.0),
        ) * self.noise_strength;

        // Apply movement
        velocity = velocity.normalize_or_zero() * self.max_speed;
        *position += velocity;

        // Optional: Keep enemies within screen bounds
        position.x = position.x.clamp(0.0, WORLD_WIDTH);
        position.y = position.y.clamp(0.0, WORLD_HEIGHT);
    }
}