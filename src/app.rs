use crate::game::GameState;

pub enum AppState {
    Start,
    Game(GameState),
    GameOver { elapsed_seconds: i32 },
}
