use macroquad::prelude::*;
use crate::player::Player;
use crate::skills::skill_id::SkillId;
use crate::enemies::EnemyView;

pub trait SkillManager {
    fn spawn(&mut self, player: &Player, enemy_views: &[EnemyView]);
    fn draw(&self);
    fn update(
        &mut self,
        delta: f32,
        player: &Player,
        enemy_views: &[EnemyView],
        on_hit: &mut dyn FnMut(SkillId, f32, usize),
    );
}