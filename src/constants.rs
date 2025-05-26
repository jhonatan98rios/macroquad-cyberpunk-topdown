use macroquad::prelude::*;
use crate::components::layout::is_mobile;

pub const ENEMIES: usize = 0;
pub const WORLD_WIDTH: f32 = 4000.0;
pub const WORLD_HEIGHT: f32 = 4000.0;

pub fn virtual_height() -> f32 {
    if is_mobile() {
        screen_height() * 0.5
    } else {
        screen_height()
    }
}

pub fn virtual_width() -> f32 {
    if is_mobile() {
        screen_width() * 0.5
    } else {
        screen_width()
    }
}