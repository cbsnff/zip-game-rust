use macroquad::prelude::*;

mod generator;
mod game;

use game::{draw_game_over_screen, draw_start_screen, GameState};

enum AppState {
    Start,
    Game(GameState),
    GameOver { elapsed_seconds: i32 },
}

fn start_new_game() -> AppState {
    AppState::Game(GameState::new(get_time()))
}

fn update(app: &mut AppState) {
    match app {
        AppState::Start => {}
        AppState::Game(game_state) => {
            if game_state.update() {
                *app = AppState::GameOver {
                    elapsed_seconds: game_state.elapsed_seconds(),
                };
            }
        }
        AppState::GameOver { .. } => {}
    }
}

fn render(app: &AppState) -> bool {
    match app {
        AppState::Start => draw_start_screen(),
        AppState::Game(game_state) => {
            game_state.draw();
            false
        }
        AppState::GameOver { elapsed_seconds } => draw_game_over_screen(*elapsed_seconds),
    }
}

#[macroquad::main("PawPath")]
async fn main() {
    let mut app = AppState::Start;

    loop {
        update(&mut app);
        let button_clicked = render(&app);

        match app {
            AppState::Start if button_clicked => app = start_new_game(),
            AppState::GameOver { .. } if button_clicked => app = start_new_game(),
            _ => {}
        }

        next_frame().await;
    }
}
