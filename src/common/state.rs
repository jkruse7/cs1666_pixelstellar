use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    WinScreen,
    EndCredits,
    Loading,

}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(AppState = AppState::InGame)]
pub enum GamePhase {
    #[default]
    Level1,
    //Add other levels here, it will cycle through them in order
}

