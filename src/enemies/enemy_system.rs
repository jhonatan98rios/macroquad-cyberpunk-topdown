use macroquad::prelude::*;
use crate::strategies::MovementStrategy;
use crate::constants::{WORLD_WIDTH, WORLD_HEIGHT};
use crate::player::Player;
use crate::strategies::CollisionStrategy;
use crate::enemies::EnemyView;
use std::cmp;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum EnemyStatus {
    Pending,
    Live,
    Dead,
}

pub enum PositionOverlap {
    Behind,
    InFront,
}

#[derive(Clone, Copy)]
pub struct EnemyData {
    pub status: EnemyStatus,
    pub last_movement: Vec2,
    pub max_health: f32,
    pub health: f32,
}

pub struct EnemySystem {
    pub positions: Vec<Vec2>,
    pub sizes: Vec<Vec2>,
    pub data: Vec<EnemyData>,
    movement_strategy: Box<dyn MovementStrategy>,
    collision_strategy: Box<dyn CollisionStrategy>,
    time: f32,
    chunk_index: usize,
    max_number_of_chunks: usize,
    texture: Option<Texture2D>,
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

        let positions = (0..count)
            .map(|_| vec2(
                rand::gen_range(0.0, WORLD_WIDTH),
                rand::gen_range(0.0, WORLD_HEIGHT),
            ))
            .collect();
            
        let sizes = vec![vec2(64.0, 64.0); count];

        let data = vec![EnemyData { 
            status: EnemyStatus::Pending,
            last_movement: Vec2::new(1.0, 0.0),
            max_health: 5.0,
            health: 5.0,
        }; count];

        EnemySystem {
            positions,
            sizes,
            data,
            movement_strategy,
            collision_strategy,
            time: 0.0,
            chunk_index: 0,
            max_number_of_chunks: 4,
            texture,
            current_frame: 0,
            frame_timer: 0.0,
            frame_duration: 0.15
        }
    }
    
    pub fn spawn_all(&mut self) {
        for data in &mut self.data {
            data.status = EnemyStatus::Live;
        }
    }
    
    pub fn update(&mut self, target_pos: Vec2, player: &mut Player) {

        self.update_movement(target_pos);
        self.update_animation_frame();

        // Here the compiler allow us to use the mutable reference to self.data
        self.collision_strategy.check_collisions(
            &mut self.positions,
            &self.sizes,
            &mut self.data,
            player
        );
    }

    fn update_movement(&mut self, target_pos: Vec2) {
        self.time += get_frame_time();
        self.chunk_index = if self.chunk_index < self.max_number_of_chunks - 1 {
            self.chunk_index + 1
        } else {
            0
        };
    
        let chunk_size = self.positions.len() / self.max_number_of_chunks;
        let start = self.chunk_index * chunk_size;
        let end = cmp::min(start + chunk_size, self.positions.len());
    
        let current_time = self.time;
        let all_positions: Vec<Vec2> = self.data
            .iter()
            .zip(&self.positions)
            .map(|(data, pos)| {
                if data.status == EnemyStatus::Live {
                    *pos
                } else {
                    Vec2::ZERO
                }
            })
            .collect();
    
        for i in start..end {
            if self.data[i].status == EnemyStatus::Live {
                let prev_pos = self.positions[i];
    
                self.movement_strategy.move_enemy(
                    &mut self.positions[i],
                    target_pos,
                    current_time,
                    i,
                    &all_positions,
                );
    
                let movement = self.positions[i] - prev_pos;
                if movement.length_squared() > 0.0 {
                    self.data[i].last_movement = movement.normalize();
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
    
    pub fn draw(&self, target_pos: Vec2, overlap: PositionOverlap) {
        match &self.texture {
            Some(texture) => {

                // Draw enemies in the specified overlap order
                let indices: Vec<usize> = (0..self.positions.len()).collect();
                
                // filter indices based on overlap
                let mut filtered_indices: Vec<usize> = indices.iter().filter(|&&i| {
                    match overlap {
                        PositionOverlap::Behind => self.positions[i].y > target_pos.y,
                        PositionOverlap::InFront => self.positions[i].y < target_pos.y,
                    }
                }).cloned().collect();

                // Sort indices by y position for correct drawing order
                filtered_indices.sort_by(|&a, &b| self.positions[b].y.partial_cmp(&self.positions[a].y).unwrap());

                for &i in &filtered_indices {
                    if self.data[i].status == EnemyStatus::Live {
                        
                        let flip_x = self.positions[i].x > target_pos.x;
                        let frame_width = self.sizes[i].x;
                        let frame_height = self.sizes[i].y;
                        
                        let params = DrawTextureParams {
                            dest_size: Some(self.sizes[i]),
                            flip_x,
                            source: Some(Rect {
                                x: self.current_frame as f32 * frame_width,
                                y: texture.height(),
                                w: frame_width,
                                h: -frame_height,
                            }),
                            ..Default::default()
                        };
                        draw_texture_ex(
                            texture,
                            self.positions[i].x,
                            self.positions[i].y,
                            WHITE,
                            params
                        );
                    }
                }
            }
            None => {
                // Fallback to rectangles if no texture
                for i in 0..self.positions.len() {
                    if self.data[i].status == EnemyStatus::Live {
                        draw_rectangle(
                            self.positions[i].x,
                            self.positions[i].y,
                            self.sizes[i].x,
                            self.sizes[i].y,
                            RED
                        );
                    }
                }
            }
        }

        self.draw_health_bars();
    }

    pub fn draw_health_bars(&self) {
        for i in 0..self.positions.len() {
            if self.data[i].status == EnemyStatus::Live {
                
                let bar_width = self.sizes[i].x / 2.0;
                let bar_height = 3.0;
                let bar_x = self.positions[i].x + (self.sizes[i].x - bar_width) / 2.0;
                let bar_y = self.positions[i].y + self.sizes[i].y + 5.0;
                
                draw_rectangle(
                    bar_x,
                    bar_y,
                    bar_width,
                    bar_height,
                    GRAY
                );
                
                let health_ratio = self.data[i].health / self.data[i].max_health;
                let health_color = Color::from_rgba(
                    ((1.0 - health_ratio) * 255.0) as u8,
                    (health_ratio * 255.0) as u8,
                    0,
                    255,
                );
                
                draw_rectangle(
                    bar_x,
                    bar_y,
                    bar_width * health_ratio,
                    bar_height,
                    health_color
                );
            }
        }
    }

    pub fn to_views(&self) -> Vec<EnemyView> {
        self.positions
            .iter()
            .zip(&self.sizes)
            .zip(&self.data)
            .map(|((&pos, &size), data)| EnemyView {
                position: pos,
                size,
                alive: data.status == EnemyStatus::Live,
            })
            .collect()
    }

    pub fn take_damage(&mut self, index: usize, damage: f32, on_die: &mut dyn FnMut(Vec2, f32)) {
        if let Some(enemy) = self.data.get_mut(index) {

            if enemy.status != EnemyStatus::Live {
                return;
            }

            enemy.health -= damage;
            if enemy.health <= 0.0 {
                enemy.status = EnemyStatus::Dead;

                on_die(
                    vec2(
                        self.positions[index].x + self.sizes[index].x / 2.0,
                        self.positions[index].y + self.sizes[index].y / 2.0,
                    ),
                    self.data[index].max_health / 5.0,
                );
            }
        }
    }
}