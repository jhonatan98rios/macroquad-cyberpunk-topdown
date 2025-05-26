use macroquad::prelude::*;
use crate::components::DrawableComponent;

pub struct TextComponent<'a> {
    text: &'a str,
    font_size: f32,
    color: Color,
    center: bool,
    x: f32,
    y: f32,
}

impl<'a> TextComponent<'a> {
    pub fn builder() -> TextComponentBuilder<'a> {
        TextComponentBuilder {
            text: None,
            font_size: None,
            color: WHITE,
            center: true,
            x: 0.0,
            y: 0.0,
        }
    }
}

impl<'a> DrawableComponent for TextComponent<'a> {
    fn draw(&mut self) {
        let measured = measure_text(self.text, None, self.font_size as u16, 1.0);

        let x = if self.center {
            self.x - measured.width / 2.0
        } else {
            self.x
        };

        draw_text(self.text, x, self.y, self.font_size, self.color);
    }
}

// ------------------ Builder ------------------

pub struct TextComponentBuilder<'a> {
    text: Option<&'a str>,
    font_size: Option<f32>,
    color: Color,
    center: bool,
    x: f32,
    y: f32,
}

impl<'a> TextComponentBuilder<'a> {
    pub fn text(mut self, text: &'a str) -> Self {
        self.text = Some(text);
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = Some(size);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    #[allow(dead_code)]
    pub fn at(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn align_center(mut self, center: bool) -> Self {
        self.center = center;
        self
    }

    pub fn build(self) -> TextComponent<'a> {
        let text = self.text.expect("TextComponent text must be set");
        let font_size = self.font_size.expect("TextComponent font_size must be set");

        TextComponent {
            text,
            font_size,
            color: self.color,
            center: self.center,
            x: self.x,
            y: self.y,
        }
    }
}