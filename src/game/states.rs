use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum ExitMenuState {
    #[default]
    False,
    Exiting,
}
