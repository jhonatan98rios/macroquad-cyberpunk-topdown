use macroquad::math::Vec2;

pub trait Renderable {
    fn position(&self) -> Vec2;
    fn draw(&self) -> ();
}