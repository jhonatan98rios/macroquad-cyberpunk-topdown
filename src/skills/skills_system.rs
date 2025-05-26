use crate::player::Player;
use crate::skills::traits::SkillManager;
use crate::skills::skill_id::SkillId;
use crate::enemies::EnemyView;
use macroquad::prelude::*;

pub struct SkillsSystem {
    skills: Vec<Box<dyn SkillManager>>,
}

impl SkillsSystem {
    pub fn new() -> Self {
        Self {
            skills: Vec::new(),
        }
    }

    pub fn add_skill(&mut self, skill: Box<dyn SkillManager>) {
        self.skills.push(skill);
    }

    pub fn spawn(&mut self, player: &Player, enemy_views: &[EnemyView]) {
        for skill in self.skills.iter_mut() {
            skill.spawn(player, enemy_views);
        }
    }

    pub fn update(
        &mut self,
        delta: f32,
        player: &Player,
        enemy_views: &[EnemyView],
        mut on_hit: impl FnMut(SkillId, f32, usize),
    ) {
        for skill in self.skills.iter_mut() {
            skill.update(delta, player, enemy_views, &mut on_hit);
        }
    }

    pub fn draw(&self) {
        for skill in self.skills.iter() {
            skill.draw();
        }
    }
}
