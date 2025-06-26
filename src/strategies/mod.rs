mod boids;
mod collisions;

pub use boids::BoidsMovement;
pub use collisions::AABBCollision;

use macroquad::prelude::*;
use crate::enemies::{Enemy, EnemyInfo};
use crate::player::Player;
use crate::scenario::collision_map::CollisionMap;

#[allow(dead_code)]
pub trait MovementStrategy: Send + Sync {
    fn move_enemy(
        &self,
        enemy: &mut Enemy,
        target: Vec2,
        time: f32,
        index: usize,
        all_enemies: &[EnemyInfo],
        obstacles: &[Rect],
    );

    fn draw(&self);
    fn get_collision_map(&self) -> CollisionMap;
}

#[allow(dead_code)]
pub trait CollisionStrategy {
    fn check_collisions(&self, enemies: &mut [Enemy], player: &mut Player);
}