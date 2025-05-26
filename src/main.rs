#[macro_use]
mod macros;

mod player;
mod enemies;
mod strategies;
mod constants;
mod menu;
mod pause;
mod state;
mod components;
mod game;
mod game_over;
mod level_up;

mod skills;
mod experience;

use macroquad::prelude::*;
use macroquad::window;

use menu::MenuScreen;
use pause::PauseScreen;
use game_over::GameOverScreen;
use level_up::LevelUpScreen;

use state::GameState;
use game::Game;
use components::joystick::Joystick;
use components::layout::is_mobile;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Macroquad WASM Game".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::Menu;
    let mut menu_screen = MenuScreen::new();
    let mut pause_screen = PauseScreen::new();
    let mut game_over_screen = GameOverScreen::new();
    let mut level_up_screen = LevelUpScreen::new();

    let joystick_size = 200.0;
    let joystick_pos_x = (screen_width() / 2.0) - (joystick_size / 5.0);
    let joystick_pos_y = screen_height() - joystick_size - 20.0;
    let joystick = if is_mobile() {
        Some(Joystick::new(vec2(joystick_pos_x, joystick_pos_y), joystick_size))
    } else {
        None
    };

    let mut game = Game::new(joystick.clone()).await;
    game.init().await;

    loop {
        match game_state {
            GameState::Menu => {
                if let Some(next_state) = menu_screen.draw() {
                    game_state = next_state;
                    game = Game::new(joystick.clone()).await;
                    game.init().await;
                }
            },
            GameState::Playing => {
                if let Some(new_state) = game.update() {
                    game_state = new_state;
                };
                game.draw_scene();
                game.draw_hub();
            },
            GameState::Paused => {    
                if let Some(next_state) = pause_screen.draw() {
                    game_state = next_state;
                }
            },
            GameState::GameOver => {
                if let Some(next_state) = game_over_screen.draw() {
                    game_state = next_state;
                }
            },
            GameState::LevelUp => {
                if let Some(next_state) = level_up_screen.draw() {
                    game_state = next_state;
                }
            },
        }

        next_frame().await;
    }
}