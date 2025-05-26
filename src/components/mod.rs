pub mod button;
pub mod text;
pub mod layout;
pub mod joystick;

pub trait DrawableComponent {
    fn draw(&mut self);
}