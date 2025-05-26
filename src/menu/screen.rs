use macroquad::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

use crate::state::GameState;
use crate::components::button::ButtonBuilder;
use crate::components::text::TextComponent;
use crate::components::layout::{Column, is_mobile};

pub struct MenuScreen<'a> {
    layout: Column<'a>,
    state_transition: Rc<RefCell<Option<GameState>>>,
}

impl<'a> MenuScreen<'a> {
    pub fn new() -> Self {

        // Using Rc<RefCell<Option<GameState>>> to allow for shared ownership and interior mutability.
        // Rc (Reference Counted) allows multiple ownership of the same data, and RefCell allows for mutable access to the data inside it.
        // This is useful for the state_transition variable, which will be shared between the MenuScreen and the button's on_click event.
        let state_transition = Rc::new(RefCell::new(None));

        // Clonning the Rc<RefCell<Option<GameState>>> to move into the closure
        // for the button's on_click event.
        // This is necessary because the closure needs to own the state_transition
        // variable, and Rc allows us to have multiple owners of the same data.
        // RefCell allows us to mutate the data inside the Rc, even though Rc itself is immutable.
        // This is a common pattern in Rust when dealing with closures and shared state.
        let state_transition_clone = Rc::clone(&state_transition);

        let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let title_size = if is_mobile() { 50.0 } else { 60.0 };

        let title = TextComponent::builder()
            .text("Welcome to Cyberpunk!")
            .font_size(title_size)
            .color(WHITE)
            .align_center(true)
            .at(screen_center.x, screen_center.y - 100.0)
            .build();

        let start_button_width = if is_mobile() { 500.0 } else { 200.0 };
        let start_button_height = if is_mobile() { 100.0 } else { 60.0 }; 

        let start_button = ButtonBuilder::new()
            .position(
                screen_center.x - start_button_width / 2.0, 
                screen_center.y,
            )
            .size(start_button_width, start_button_height)
            .label("Start Game")
            .on_click(move || {
                *state_transition_clone.borrow_mut() = Some(GameState::Playing);
            })
            .color(Color::from_rgba(90, 20, 20, 255))
            .hover_color(Color::from_rgba(60, 20, 20, 255))
            .build();

        let layout = Column::new()
            .centered()
            .spacing(20.0)
            .add_child(Box::new(title))
            .add_child(Box::new(start_button));

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
