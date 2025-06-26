use super::MovementStrategy;
use macroquad::prelude::*;
use crate::constants::{WORLD_HEIGHT, WORLD_WIDTH};
use crate::enemies::{Enemy, EnemyInfo, EnemyStatus};


// Produto vetorial 2D: retorna escalar
trait PerpDot {
    fn perp_dot(self, other: Vec2) -> f32;
}
impl PerpDot for Vec2 {
    fn perp_dot(self, other: Vec2) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

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

impl BoidsMovement {
    fn line_intersection(&self, p1: Vec2, p2: Vec2, q1: Vec2, q2: Vec2) -> Option<Vec2> {
        let r = p2 - p1;
        let s = q2 - q1;
        let denom = r.perp_dot(s);

        if denom.abs() < f32::EPSILON {
            return None; // paralelas ou colineares
        }

        let t = (q1 - p1).perp_dot(s) / denom;
        let u = (q1 - p1).perp_dot(r) / denom;

        if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
            Some(p1 + r * t)
        } else {
            None
        }
    }


    fn ray_intersects_rect(&self, start: Vec2, end: Vec2, rect: Rect) -> Option<Vec2> {
        let mut closest_hit: Option<(f32, Vec2)> = None;

        let rect_edges = [
            (vec2(rect.x, rect.y), vec2(rect.x + rect.w, rect.y)), // topo
            (vec2(rect.x + rect.w, rect.y), vec2(rect.x + rect.w, rect.y + rect.h)), // direita
            (vec2(rect.x + rect.w, rect.y + rect.h), vec2(rect.x, rect.y + rect.h)), // baixo
            (vec2(rect.x, rect.y + rect.h), vec2(rect.x, rect.y)), // esquerda
        ];

        for (p1, p2) in rect_edges {
            if let Some(hit) = self.line_intersection(start, end, p1, p2) {
                let dist = start.distance_squared(hit);
                if closest_hit.is_none() || dist < closest_hit.unwrap().0 {
                    closest_hit = Some((dist, hit));
                }
            }
        }

        closest_hit.map(|(_, point)| point)
    }


    fn find_clear_path(&self, start: Vec2, target: Vec2, enemy_size: Vec2, obstacles: &[Rect]) -> Vec2 {
        let direction = (target - start).normalize_or_zero();
        let ray_end = start + direction * 1000.0;

        let mut closest_hit: Option<Rect> = None;

        for rect in obstacles {
            if let Some(_) = self.ray_intersects_rect(start, ray_end, *rect) {
                closest_hit = Some(*rect);
                break;
            }
        }

        if let Some(_) = closest_hit {

            let mut best_target = target;
            let mut best_distance = f32::MAX;

            let offsets = [
                vec2(-(enemy_size.x / 2.0), 0.0), // esquerda
                vec2(enemy_size.x / 2.0, 0.0),  // direita
                vec2(0.0, -(enemy_size.y / 2.0)), // cima
                vec2(0.0, enemy_size.y / 2.0),  // baixo
            ];

            for offset in offsets {
                let test_pos = start + offset;
                let test_rect = Rect::new(test_pos.x, test_pos.y, enemy_size.x, enemy_size.y);

                let collides = obstacles.iter().any(|ob| ob.overlaps(&test_rect));

                if !collides {
                    let dist = test_pos.distance(target);
                    if dist < best_distance {
                        best_distance = dist;
                        best_target = test_pos;
                    }
                }
            }

            return best_target;
        }

        target
    }



}

impl MovementStrategy for BoidsMovement {
    fn move_enemy(
        &self,
        enemy: &mut Enemy,
        target: Vec2,
        _time: f32,
        index: usize,
        all_enemies: &[EnemyInfo],
        obstacles: &[Rect],
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

        let path_target = self.find_clear_path(enemy.position, target, enemy.size, obstacles);
        let to_player = path_target - enemy.position;

        let player_dist = to_player.length();
        let player_influence = 1.0 - (player_dist / self.player_distance).min(1.0).max(0.0);

        velocity += to_player.normalize_or_zero() * self.player_weight * player_influence.powf(0.5);


        // >>> Obstacles (repulsão) <<<
        let mut obstacle_avoidance = Vec2::ZERO;
        let safe_distance = 40.0;

        for obstacle in obstacles {
            let closest = vec2(
                enemy.position.x.clamp(obstacle.left(), obstacle.right()),
                enemy.position.y.clamp(obstacle.top(), obstacle.bottom()),
            );

            let dist = enemy.position.distance(closest);

            if dist < safe_distance {
                let force = (1.0 - (dist / safe_distance)).powf(2.0);
                obstacle_avoidance += (enemy.position - closest).normalize_or_zero() * force;
            }
        }

        obstacle_avoidance = obstacle_avoidance.normalize_or_zero() * self.separation_weight;
        velocity += obstacle_avoidance;



        // Adiciona um pouco de ruído ao movimento
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