use macroquad::prelude::*;
use crate::components::layout::is_mobile;

#[derive(Clone)]
pub struct Joystick {
    pub base_pos: Vec2,
    pub radius: f32,
    pub knob_radius: f32,
    pub dragging: bool,
    pub knob_pos: Vec2,
    pub touch_id: Option<u64>,
}

impl Joystick {
    pub fn new(base_pos: Vec2, radius: f32) -> Self {
        Self {
            base_pos,
            radius,
            knob_radius: radius * 0.4,
            dragging: false,
            knob_pos: base_pos,
            touch_id: None,
        }
    }

    pub fn update(&mut self) {
        if !is_mobile() {
            return;
        }

        let all_touches = touches();

        // Find a valid touch
        for touch in &all_touches {
            let touch_pos = vec2(touch.position.x, touch.position.y);

            if self.touch_id.is_none() && touch.phase == TouchPhase::Started {
                // Start dragging if touch began inside the joystick base
                if touch_pos.distance(self.base_pos) < self.radius {
                    self.dragging = true;
                    self.touch_id = Some(touch.id);
                    self.knob_pos = touch_pos;
                }
            }

            if Some(touch.id) == self.touch_id {
                match touch.phase {
                    TouchPhase::Moved | TouchPhase::Stationary => {
                        let direction = (touch_pos - self.base_pos).clamp_length_max(self.radius);
                        self.knob_pos = self.base_pos + direction;
                    }
                    TouchPhase::Ended | TouchPhase::Cancelled => {
                        self.dragging = false;
                        self.touch_id = None;
                        self.knob_pos = self.base_pos;
                    }
                    _ => {}
                }
            }
        }

        // Reset knob if no valid touches
        if self.touch_id.is_some() && !all_touches.iter().any(|t| t.id == self.touch_id.unwrap()) {
            self.dragging = false;
            self.touch_id = None;
            self.knob_pos = self.base_pos;
        }
    }

    pub fn draw(&self) {
        if is_mobile() {
            draw_circle(self.base_pos.x, self.base_pos.y, self.radius, Color::from_rgba(0, 0, 0, 100));
            draw_circle(self.knob_pos.x, self.knob_pos.y, self.knob_radius, Color::from_rgba(255, 255, 255, 180));
        }
    }

    pub fn direction(&self) -> Vec2 {
        if self.dragging {
            let dir = self.knob_pos - self.base_pos;
            if dir.length() > 5.0 {
                dir.normalize()
            } else {
                vec2(0.0, 0.0)
            }
        } else {
            vec2(0.0, 0.0)
        }
    }
}
