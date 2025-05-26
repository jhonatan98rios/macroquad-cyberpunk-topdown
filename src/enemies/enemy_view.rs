use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct EnemyView {
    pub position: Vec2,
    pub size: Vec2,
    pub alive: bool,
}