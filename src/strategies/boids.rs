use super::MovementStrategy;
use crate::constants::{WORLD_HEIGHT, WORLD_WIDTH};
use crate::enemies::{Enemy, EnemyInfo, EnemyStatus};
use crate::scenario::collision_map::CollisionMap;
use macroquad::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

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
    pub collision_map: CollisionMap,
}

impl MovementStrategy for BoidsMovement {
    fn move_enemy(
        &self,
        enemy: &mut Enemy,
        target: Vec2,
        _time: f32,
        index: usize,
        all_enemies: &[EnemyInfo],
        _obstacles: &[Rect],
    ) {
        let tile_size = 64.0;
        let current_tile = self.world_to_tile(enemy.position, tile_size).unwrap_or((0, 0));

        if current_tile != enemy.last_tile || enemy.path.is_empty() {
            enemy.path = self.find_path(enemy.position, target, tile_size);
            enemy.path_index = 0;
            enemy.last_tile = current_tile;
        }

        let path_target = enemy.path.get(enemy.path_index).copied().unwrap_or(target);

        if enemy.position.distance(path_target) < 4.0 && enemy.path_index + 1 < enemy.path.len() {
            enemy.path_index += 1;
        }

        // BOIDS
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
                    let force = (1.0 - (dist / self.separation_dist)).powf(2.0);
                    separation += (enemy.position - other.position).normalize() * force;
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

        let to_target = path_target - enemy.position;
        velocity += to_target.normalize_or_zero() * self.player_weight;

        velocity += Vec2::new(
            rand::gen_range(-1.0, 1.0),
            rand::gen_range(-1.0, 1.0),
        ) * self.noise_strength;

        velocity = velocity.normalize_or_zero() * self.max_speed;
        let new_pos = enemy.position + velocity;

        if !self.check_collision_pixel(new_pos, enemy.size, tile_size) {
            enemy.position = new_pos;

            if velocity.length_squared() > 0.0 {
                enemy.last_movement = velocity.normalize();
            }
        } else {
            enemy.path.clear(); // força recalcular o caminho
        }

        enemy.position.x = enemy.position.x.clamp(0.0, WORLD_WIDTH);
        enemy.position.y = enemy.position.y.clamp(0.0, WORLD_HEIGHT);
    }

    fn draw(&self) {
        self.draw();
    }

    fn get_collision_map(&self) -> CollisionMap {
        return self.collision_map.clone();
    }
}


impl BoidsMovement {
    fn is_solid(&self, map: &CollisionMap, x: usize, y: usize) -> bool {
        if x >= map.width || y >= map.height {
            return true;
        }
        map.tiles[y * map.width + x] != 0
    }

    fn world_to_tile(&self, pos: Vec2, tile_size: f32) -> Option<(usize, usize)> {
        let x = (pos.x / tile_size).floor() as usize;
        let y = (pos.y / tile_size).floor() as usize;
        Some((x, y))
    }

    fn check_collision_pixel(&self, pos: Vec2, size: Vec2, tile_size: f32) -> bool {
        let left = (pos.x / tile_size).floor() as isize;
        let right = ((pos.x + size.x) / tile_size).floor() as isize;
        let top = (pos.y / tile_size).floor() as isize;
        let bottom = ((pos.y + size.y) / tile_size).floor() as isize;

        for ty in top..=bottom {
            for tx in left..=right {
                if tx < 0 || ty < 0 {
                    return true;
                }

                let tx = tx as usize;
                let ty = ty as usize;

                if tx >= self.collision_map.width || ty >= self.collision_map.height {
                    return true;
                }

                if self.is_solid(&self.collision_map, tx, ty) {
                    return true;
                }
            }
        }

        false
    }

    fn heuristic(&self, a: (usize, usize), b: (usize, usize)) -> usize {
        (a.0 as isize - b.0 as isize).abs() as usize + (a.1 as isize - b.1 as isize).abs() as usize
    }

    fn find_nearest_walkable(&self, start: (usize, usize)) -> Option<(usize, usize)> {
        let deltas = [
            (0, 0), (1, 0), (0, 1), (-1, 0), (0, -1),
            (1, 1), (-1, 1), (-1, -1), (1, -1),
            (2, 0), (0, 2), (-2, 0), (0, -2),
        ];

        for (dx, dy) in deltas {
            let nx = start.0 as isize + dx;
            let ny = start.1 as isize + dy;
            if nx < 0 || ny < 0 {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if nx < self.collision_map.width && ny < self.collision_map.height
                && !self.is_solid(&self.collision_map, nx, ny)
            {
                return Some((nx, ny));
            }
        }

        None
    }

    fn find_path(&self, start_pos: Vec2, goal_pos: Vec2, tile_size: f32) -> Vec<Vec2> {
        let start = self.world_to_tile(start_pos, tile_size).unwrap_or((0, 0));

        let mut goal = self.world_to_tile(goal_pos, tile_size).unwrap_or((0, 0));
        if self.is_solid(&self.collision_map, goal.0, goal.1) {
            if let Some(nearest) = self.find_nearest_walkable(goal) {
                goal = nearest;
            }
        }

        #[derive(Copy, Clone, Eq, PartialEq)]
        struct Node {
            pos: (usize, usize),
            cost: usize,
            priority: usize,
        }

        impl Ord for Node {
            fn cmp(&self, other: &Self) -> Ordering {
                other.priority.cmp(&self.priority)
            }
        }

        impl PartialOrd for Node {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut frontier = BinaryHeap::new();
        frontier.push(Node {
            pos: start,
            cost: 0,
            priority: 0,
        });

        let mut came_from: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
        let mut cost_so_far: HashMap<(usize, usize), usize> = HashMap::new();

        came_from.insert(start, None);
        cost_so_far.insert(start, 0);

        let neighbors = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some(current) = frontier.pop() {
            if current.pos == goal {
                break;
            }

            for (dx, dy) in neighbors.iter() {
                let next = (
                    (current.pos.0 as isize + dx) as usize,
                    (current.pos.1 as isize + dy) as usize,
                );

                if self.is_solid(&self.collision_map, next.0, next.1) {
                    continue;
                }

                let new_cost = cost_so_far[&current.pos] + 1;
                if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                    cost_so_far.insert(next, new_cost);
                    let priority = new_cost + self.heuristic(goal, next);
                    frontier.push(Node {
                        pos: next,
                        cost: new_cost,
                        priority,
                    });
                    came_from.insert(next, Some(current.pos));
                }
            }
        }

        // Reconstrói o caminho
        let mut path = vec![];
        let mut current = goal;

        while let Some(Some(prev)) = came_from.get(&current) {
            path.push(Vec2::new(
                current.0 as f32 * tile_size + tile_size / 2.0,
                current.1 as f32 * tile_size + tile_size / 2.0,
            ));
            current = *prev;
        }

        path.reverse();
        path
    }

    pub fn draw(&self) {
        let tile_size = 64.0;
        for y in 0..self.collision_map.height {
            for x in 0..self.collision_map.width {
                let tile = self.collision_map.tiles[y * self.collision_map.width + x];
                let color = if tile == 1 { RED } else { GREEN };
                draw_rectangle(
                    x as f32 * tile_size,
                    y as f32 * tile_size,
                    tile_size,
                    tile_size,
                    color,
                );
            }
        }
    }
}
