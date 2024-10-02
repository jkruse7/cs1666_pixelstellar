use bevy::prelude::*;
use bevy::input::keyboard::KeyCode; 

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    Paused,
    Setting,
    DevLab,
}

impl AppState {
    fn next(&self) -> Self {
        match *self {
            AppState::MainMenu => AppState::InGame,
            AppState::InGame => AppState::Paused,
            AppState::Paused => AppState::InGame,
            AppState::Setting => AppState::MainMenu,
            AppState::DevLab => AppState::MainMenu,
        }
    }
}

// Function to initialize the default state to MainMenu
pub fn setup_app_state(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::MainMenu);
}



pub fn pause_game(
    input: Res<ButtonInput<KeyCode>>, // Use Input<KeyCode> for keyboard input
    mut next_state: ResMut<NextState<AppState>>,
    current_state: Res<State<AppState>>, // State<AppState> wrapper
) {
    if input.just_pressed(KeyCode::Escape) {
        match current_state.get() { // Use .get() to access the current state
            AppState::InGame | AppState::Paused => {
                next_state.set(current_state.get().next());
            }
            _ => {}
        }
    }
}