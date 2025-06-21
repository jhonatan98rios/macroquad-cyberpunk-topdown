use macroquad::math::Vec2;
use macroquad::prelude::*;

pub trait Renderable {
    fn position(&self) -> Vec2;
    fn draw(&self) -> ();
}