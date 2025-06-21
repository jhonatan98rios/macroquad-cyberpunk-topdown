use macroquad::prelude::*;
use std::cmp;

use crate::strategies::{MovementStrategy, CollisionStrategy};
use crate::constants::{WORLD_WIDTH, WORLD_HEIGHT};
use crate::player::Player;
use crate::enemies::{
    Enemy, EnemyInfo, EnemyRenderable, EnemyStatus, EnemyView
};

pub struct EnemySystem {
    pub enemies: Vec<Enemy>,
    movement_strategy: Box<dyn MovementStrategy>,
    collision_strategy: Box<dyn CollisionStrategy>,
    time: f32,
    chunk_index: usize,
    max_number_of_chunks: usize,
    pub texture: Option<Texture2D>,
    current_frame: usize,
    frame_timer: f32,
    frame_duration: f32,
}

impl EnemySystem {
    pub async fn new(
        count: usize, 
        movement_strategy: Box<dyn MovementStrategy>,
        collision_strategy: Box<dyn CollisionStrategy>,
    ) -> Self {

        let texture = match load_texture("images/enemy_spritesheet.png").await {
            Ok(t) => Some(t),
            Err(_) => {
                println!("Failed to load enemy texture, falling back to rectangles");
                None
            }
        };

        let enemies = (0..count)
            .map(|_| Enemy {
                position: vec2(rand::gen_range(0.0, WORLD_WIDTH), rand::gen_range(0.0, WORLD_HEIGHT)),
                size: vec2(64.0, 64.0),
                status: EnemyStatus::Pending,
                last_movement: Vec2::new(1.0, 0.0),
                max_health: 5.0,
                health: 5.0,
            })
            .collect();

        EnemySystem {
            enemies,
            movement_strategy,
            collision_strategy,
            time: 0.0,
            chunk_index: 0,
            max_number_of_chunks: 4,
            texture,
            current_frame: 0,
            frame_timer: 0.0,
            frame_duration: 0.15,
        }
    }

    pub fn spawn_all(&mut self) {
        for enemy in &mut self.enemies {
            enemy.status = EnemyStatus::Live;
        }
    }

    pub fn update(&mut self, target_pos: Vec2, player: &mut Player) {

        self.update_movement(target_pos);
        self.update_animation_frame();

        // Here the compiler allow us to use the mutable reference to self.data
        self.collision_strategy.check_collisions(&mut self.enemies, player);
    }

    fn update_movement(&mut self, target_pos: Vec2) {
        self.time += get_frame_time();
        self.chunk_index = (self.chunk_index + 1) % self.max_number_of_chunks;

        let chunk_size = self.enemies.len() / self.max_number_of_chunks;
        let start = self.chunk_index * chunk_size;
        let end = cmp::min(start + chunk_size, self.enemies.len());

        let current_time = self.time;

        let all_enemies: Vec<EnemyInfo> = self.enemies
            .iter()
            .map(|e| EnemyInfo {
                position: e.position,
                status: e.status,
            })
            .collect();

        for (i, enemy) in self.enemies[start..end].iter_mut().enumerate() {
            if enemy.status == EnemyStatus::Live {
                let prev_pos = enemy.position;
                let index = start + i;

                self.movement_strategy.move_enemy(
                    enemy,
                    target_pos,
                    current_time,
                    index,
                    &all_enemies,
                );

                let movement = enemy.position - prev_pos;
                if movement.length_squared() > 0.0 {
                    enemy.last_movement = movement.normalize();
                }
            }
        }
    }

    fn update_animation_frame(&mut self) {
        self.frame_timer += get_frame_time();
        if self.frame_timer >= self.frame_duration {
            self.frame_timer = 0.0;
            self.current_frame = (self.current_frame + 1) % 4;
        }
    }

    pub fn take_damage(&mut self, index: usize, damage: f32, on_die: &mut dyn FnMut(Vec2, f32)) {
        if let Some(enemy) = self.enemies.get_mut(index) {
            if enemy.status != EnemyStatus::Live {
                return;
            }

            enemy.health -= damage;
            if enemy.health <= 0.0 {
                enemy.status = EnemyStatus::Dead;

                on_die(
                    enemy.position + enemy.size / 2.0,
                    enemy.max_health / 5.0,
                );
            }
        }
    }

    pub fn to_views(&self) -> Vec<EnemyView> {
        self.enemies.iter()
            .map(|e| EnemyView {
                position: e.position,
                size: e.size,
                alive: e.status == EnemyStatus::Live,
            })
            .collect()
    }

    pub fn to_renderables(&self, player_pos: Vec2) -> Vec<EnemyRenderable<'_>> {
        let Some(texture) = &self.texture else { return vec![] };

        self.enemies
            .iter()
            .filter(|e| e.status == EnemyStatus::Live)
            .map(|enemy| EnemyRenderable { 
                enemy, 
                texture, 
                current_frame: self.current_frame,
                flip_x: enemy.position.x > player_pos.x,
            })
            .collect()
    }

}
