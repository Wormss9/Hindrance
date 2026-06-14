mod exit;
mod game;
mod main;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            main::MainMenuPlugin,
            game::GamePlugin,
            exit::ExitMenuPlugin,
        ));
    }
}
