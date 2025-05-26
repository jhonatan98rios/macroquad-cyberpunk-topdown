use macroquad::prelude::*;
use crate::components::DrawableComponent;
use crate::components::layout::{ is_mobile };


pub struct Button<'a> {
    rect: Rect,
    label: &'a str,
    on_click: Box<dyn FnMut() + 'a>,
    color: Color,
    hover_color: Color,
}

impl<'a> Button<'a> {
    pub fn draw(&mut self) {
        let mouse = mouse_position();
        let is_hovering = self.rect.contains(vec2(mouse.0, mouse.1));

        let bg_color = if is_hovering {
            self.hover_color
        } else {
            self.color
        };

        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, bg_color);

        let font_size = if is_mobile() { 50.0 } else { 30.0 };
        let text_dim = measure_text(self.label, None, font_size as u16, 1.0);
        draw_text(
            self.label,
            self.rect.x + (self.rect.w - text_dim.width) / 2.0,
            self.rect.y + (self.rect.h + text_dim.height) / 2.0,
            font_size,
            WHITE,
        );

        if is_hovering && is_mouse_button_pressed(MouseButton::Left) {
            (self.on_click)();
        }
    }
}

impl<'a> DrawableComponent for Button<'a> {
    fn draw(&mut self) {
        self.draw();
    }
}

// ------------------ Builder ------------------

pub struct ButtonBuilder<'a> {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    label: Option<&'a str>,
    on_click: Box<dyn FnMut() + 'a>,
    color: Color,
    hover_color: Color,
}

impl<'a> ButtonBuilder<'a> {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 40.0,
            label: None,
            on_click: Box::new(|| {}),
            color: Color::from_rgba(100, 20, 20, 255),
            hover_color: Color::from_rgba(200, 50, 50, 255),
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn hover_color(mut self, color: Color) -> Self {
        self.hover_color = color;
        self
    }

    pub fn on_click(mut self, handler: impl FnMut() + 'a) -> Self {
        self.on_click = Box::new(handler);
        self
    }

    pub fn build(self) -> Button<'a> {
        let label = self.label.expect("Button label must be set");

        Button {
            rect: Rect::new(self.x, self.y, self.width, self.height),
            label,
            on_click: self.on_click,
            color: self.color,
            hover_color: self.hover_color,
        }
    }
}
