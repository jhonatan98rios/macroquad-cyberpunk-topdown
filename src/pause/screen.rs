use macroquad::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

use crate::state::GameState;
use crate::components::button::ButtonBuilder;
use crate::components::text::TextComponent;
use crate::components::layout::{Column, is_mobile};

pub struct PauseScreen<'a> {
    layout: Column<'a>,
    state_transition: Rc<RefCell<Option<GameState>>>,
}

impl<'a> PauseScreen<'a> {
    pub fn new() -> Self {

        // Using Rc<RefCell<Option<GameState>>> to allow for shared ownership and interior mutability.
        // Rc (Reference Counted) allows multiple ownership of the same data, and RefCell allows for mutable access to the data inside it.
        // This is useful for the state_transition variable, which will be shared between the PauseScreen and the button's on_click event.
        let state_transition = Rc::new(RefCell::new(None));
        let state_resume = Rc::clone(&state_transition);
        let state_menu = Rc::clone(&state_transition);

        let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let title_size = if is_mobile() { 50.0 } else { 60.0 };

        let title = TextComponent::builder()
            .text("Paused")
            .font_size(title_size)
            .color(WHITE)
            .align_center(true)
            .at(screen_center.x, screen_center.y - 100.0)
            .build();

        let button_width = if is_mobile() { 500.0 } else { 200.0 };
        let button_height = if is_mobile() { 100.0 } else { 60.0 };

        let resume_button = ButtonBuilder::new()
            .position(
                screen_center.x - button_width / 2.0, 
                screen_center.y,
            )
            .size(button_width, button_height)
            .label("Resume Game")
            .on_click(move || {
                *state_resume.borrow_mut() = Some(GameState::Playing);
            })
            .color(Color::from_rgba(90, 20, 20, 255))
            .hover_color(Color::from_rgba(60, 20, 20, 255))
            .build();

        let exit_button = ButtonBuilder::new()
            .position(
                screen_center.x - button_width / 2.0, 
                if is_mobile() { screen_center.y + 120.0 } else { screen_center.y + 80.0 },
            )
            .size(button_width, button_height)
            .label("Exit Game")
            .on_click(move || {
                *state_menu.borrow_mut() = Some(GameState::Menu);
            })
            .color(Color::from_rgba(90, 20, 20, 255))
            .hover_color(Color::from_rgba(60, 20, 20, 255))
            .build();

        let layout = Column::new()
            .centered()
            .spacing(20.0)
            .add_child(Box::new(title))
            .add_child(Box::new(resume_button))
            .add_child(Box::new(exit_button));

        if is_key_pressed(KeyCode::Escape) {
            return Self {
                layout,
                state_transition: Rc::new(RefCell::new(Some(GameState::Playing))),
            }
        }

        Self {
            layout,
            state_transition,
        }
    }

    pub fn draw(&mut self) -> Option<GameState> {
        clear_background(Color::from_rgba(30, 30, 30, 255));        
        self.layout.draw();
        self.state_transition.borrow_mut().take()
    }
}
