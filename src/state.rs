use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum ScreenState {
    #[default]
    MainMenu,
    InLobby,
    BrowsingLobbies,
    InGame,
}
