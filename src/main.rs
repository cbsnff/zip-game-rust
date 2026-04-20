use macroquad::prelude::*;

mod app;
mod game;

use app::AppState;
use game::{
    GameState, draw_game_over_screen, draw_start_screen, play_again_button_clicked,
    start_button_clicked,
};

fn update(app: &mut AppState) {
    match app {
        AppState::Start => {
            if start_button_clicked() {
                *app = AppState::Game(GameState::new(get_time()));
            }
        }
        AppState::Game(game_state) => {
            if game_state.update() {
                *app = AppState::GameOver {
                    elapsed_seconds: game_state.elapsed_seconds(),
                };
            }
        }
        AppState::GameOver { .. } => {
            if play_again_button_clicked() {
                *app = AppState::Game(GameState::new(get_time()));
            }
        }
    }
}

fn render(app: &AppState) {
    match app {
        AppState::Start => draw_start_screen(),
        AppState::Game(game_state) => game_state.draw(),
        AppState::GameOver { elapsed_seconds } => draw_game_over_screen(*elapsed_seconds),
    }
}

#[macroquad::main("Zip")]
async fn main() {
    let mut app = AppState::Start;

    loop {
        update(&mut app);
        render(&app);
        next_frame().await;
    }
}
