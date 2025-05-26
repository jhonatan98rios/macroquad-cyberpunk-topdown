use macroquad::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

use crate::state::GameState;
use crate::components::button::ButtonBuilder;
use crate::components::text::TextComponent;
use crate::components::layout::{Column, is_mobile};
use super::{get_random_skills};

pub struct LevelUpScreen<'a> {
    layout: Column<'a>,
    state_transition: Rc<RefCell<Option<GameState>>>,
}

impl<'a> LevelUpScreen<'a> {
    pub fn new() -> Self {

        // Using Rc<RefCell<Option<GameState>>> to allow for shared ownership and interior mutability.
        // Rc (Reference Counted) allows multiple ownership of the same data, and RefCell allows for mutable access to the data inside it.
        // This is useful for the state_transition variable, which will be shared between the LevelUpScreen and the button's on_click event.
        let state_transition = Rc::new(RefCell::new(None));

        let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let title_size = if is_mobile() { 50.0 } else { 60.0 };

        let title = TextComponent::builder()
            .text("Level UP!")
            .font_size(title_size)
            .color(WHITE)
            .align_center(true)
            .at(screen_center.x, screen_center.y - 100.0)
            .build();


        let skills = get_random_skills();

        let button_width = if is_mobile() { 500.0 } else { 200.0 };
        let button_height = if is_mobile() { 100.0 } else { 60.0 };
        let mut button_y = screen_center.y;

        let mut buttons = vec![];
        for skill in skills {
            let state_transition_clone = Rc::clone(&state_transition);
            let button = ButtonBuilder::new()
                .position(
                    screen_center.x - button_width / 2.0, 
                    button_y,
                )
                .size(button_width, button_height)
                .label(skill)
                .on_click(move || {
                    *state_transition_clone.borrow_mut() = Some(GameState::Playing);
                    println!("Selected skill: {}", skill);
                })
                .color(Color::from_rgba(90, 20, 20, 255))
                .hover_color(Color::from_rgba(60, 20, 20, 255))
                .build();
            buttons.push(button);
            button_y += button_height + 20.0;
        }

        let mut column = Column::new()
            .centered()
            .spacing(20.0)
            .add_child(Box::new(title));

        for button in buttons {
            column = column.add_child(Box::new(button));
        }

        let layout = column;

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
