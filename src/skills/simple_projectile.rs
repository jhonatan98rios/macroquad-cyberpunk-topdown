use macroquad::prelude::*;
use crate::player::Player;
use super::traits::{SkillManager};
use std::collections::HashMap;
use crate::skills::skill_id::SkillId;
use crate::enemies::EnemyView;

pub struct SimpleProjectile {
    pub id: SkillId,
    pub position: Vec2,
    pub direction: Vec2,
    pub speed: f32,
    pub radius: f32,
    pub life: f32,
    pub active: bool,
}

impl SimpleProjectile {
    fn update(
        &mut self,
        delta: f32,
        enemy_views: &[EnemyView],
        on_hit: &mut dyn FnMut(usize)
    ) {
        if !self.active {
            return;
        }

        // Movement
        self.position += self.direction * self.speed * delta;

        // Collision detection
        for (enemy_index, enemy) in enemy_views.iter().enumerate() {
            if !enemy.alive {
                continue;
            }

            let closest_x = self.position.x.clamp(enemy.position.x, enemy.position.x + enemy.size.x);
            let closest_y = self.position.y.clamp(enemy.position.y, enemy.position.y + enemy.size.y);
            let distance = vec2(closest_x - self.position.x, closest_y - self.position.y).length();

            if distance < self.radius {
                on_hit(enemy_index);
                self.active = false;
                break;
            }
        }

        self.life -= delta;
        if self.life <= 0.0 {
            self.active = false;
        }
    }

    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, PURPLE);
    }

    fn is_expired(&self) -> bool {
        !self.active || self.life <= 0.0
    }
}

pub struct SimpleProjectileManager {
    pub projectiles: HashMap<SkillId, SimpleProjectile>,
    pub damage: f32,
    pub cooldown: f32,
    pub timer: f32,
    pub speed: f32,
    pub radius: f32,
    pub life: f32,
    pub is_active: bool
}

impl SkillManager for SimpleProjectileManager {
    fn spawn(&mut self, player: &Player, enemy_views: &[EnemyView]) {

        if !self.is_active {
            return;
        }

        if self.timer > 0.0 {
            return;
        }

        if let Some(target) = enemy_views
            .iter()
            .filter(|e| e.alive)
            .min_by(|a, b| {
                let da = player.position().distance(a.position);
                let db = player.position().distance(b.position);
                da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
            }) {
            let dir = (target.position - player.position()).normalize_or_zero();

            let projectile = SimpleProjectile {
                id: SkillId::new(),
                position: Vec2::new(
                    player.x + player.size * 0.5,
                    player.y + player.size * 0.5,
                ),
                direction: dir,
                speed: self.speed,
                radius: self.radius,
                life: self.life,
                active: true,
            };

            self.projectiles.insert(projectile.id, projectile);
            self.timer = self.cooldown;
        }
    }

    fn update(
        &mut self,
        delta: f32,
        _: &Player,
        enemy_views: &[EnemyView],
        on_hit: &mut dyn FnMut(SkillId, f32, usize),
    ) {
        self.timer = (self.timer - delta).max(0.0);

        let mut expired_ids = vec![];

        for (&skill_id, projectile) in self.projectiles.iter_mut() {
            projectile.update(delta, enemy_views, &mut |enemy_index| {
                on_hit(skill_id, self.damage, enemy_index);
            });

            if projectile.is_expired() {
                expired_ids.push(skill_id);
            }
        }

        for id in expired_ids {
            self.projectiles.remove(&id);
        }
    }

    fn draw(&self) {
        for projectile in self.projectiles.values() {
            projectile.draw();
        }
    }
}
