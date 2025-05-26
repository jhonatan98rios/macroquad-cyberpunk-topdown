mod boids;
mod collisions;

pub use boids::BoidsMovement;
pub use collisions::AABBCollision;

use macroquad::prelude::*;
use crate::enemies::{EnemyData};
use crate::player::Player;

#[allow(dead_code)]
pub trait MovementStrategy: Send + Sync {
    fn move_enemy(
        &self, 
        position: &mut Vec2,
        target: Vec2,
        time: f32,
        index: usize,
        all_positions: &[Vec2]
    );
}

#[allow(dead_code)]
pub trait CollisionStrategy {
    fn check_collisions(
        &mut self,
        positions: &mut Vec<Vec2>,
        sizes: &Vec<Vec2>,
        data: &mut Vec<EnemyData>,
        player: &mut Player
    );
}