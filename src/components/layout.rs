use macroquad::prelude::*;
use crate::components::DrawableComponent;

pub struct Column<'a> {
    children: Vec<Box<dyn DrawableComponent + 'a>>,
    center: bool,
    spacing: f32,
}

impl<'a> Column<'a> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            center: false,
            spacing: 10.0,
        }
    }

    pub fn centered(mut self) -> Self {
        self.center = true;
        self
    }

    pub fn spacing(mut self, value: f32) -> Self {
        self.spacing = value;
        self
    }

    pub fn add_child(mut self, child: Box<dyn DrawableComponent + 'a>) -> Self {
        self.children.push(child);
        self
    }

    pub fn draw(&mut self) {
        for child in self.children.iter_mut() {
            child.draw();
        }
    }
}

pub fn is_mobile() -> bool {
    let width = screen_width();
    let height = screen_height();
    let aspect_ratio = height / width;
    return aspect_ratio > 1.3;
}