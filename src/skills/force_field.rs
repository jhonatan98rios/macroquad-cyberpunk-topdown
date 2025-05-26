use macroquad::prelude::*;
use crate::player::Player;
use super::traits::{SkillManager};
use std::collections::HashMap;
use crate::skills::skill_id::SkillId;
use crate::enemies::EnemyView;

pub struct ForceFieldUnit {
    pub id: SkillId,
    pub position: Vec2,
    pub radius: f32,
}

pub struct ForceFieldManager {
    pub projectiles: HashMap<SkillId, ForceFieldUnit>,
    pub damage: f32,
    pub radius: f32,
    pub is_active: bool,
    pub initialized: bool,
}

impl SkillManager for ForceFieldManager {

    fn spawn(&mut self, player: &Player, _: &[EnemyView]) {

        if !self.is_active {
            return;
        }

        if self.initialized {
            return;
        }

        self.initialized = true;

        let projectile = ForceFieldUnit {
            id: SkillId::new(),
            position: Vec2::new(
                player.x + (player.size * 0.5),
                player.y + (player.size * 0.5),
            ),
            radius: self.radius,
        };

        self.projectiles.insert(projectile.id, projectile);
    }

    fn update(
        &mut self,
        _: f32,
        player: &Player,
        enemy_views: &[EnemyView],
        on_hit: &mut dyn FnMut(SkillId, f32, usize),
    ) {
        for (&skill_id, projectile) in self.projectiles.iter_mut() {

            projectile.position = Vec2::new(
                player.x + (player.size * 0.5),
                player.y + (player.size * 0.5),
            );
            
            for (enemy_index, enemy) in enemy_views.iter().enumerate() {
                if projectile.position.distance(enemy.position) <= self.radius {
                    on_hit(skill_id, self.damage, enemy_index);
                }
            }
        }
    }

    fn draw(&self) {
        for projectile in self.projectiles.values() {
            draw_circle(projectile.position.x, projectile.position.y, projectile.radius, Color::from_rgba(100, 100, 255, 25));
        }
    }
}
