pub mod colors;
mod error;
pub mod exit_menu;
mod game;
pub mod game_logic;
mod main_menu;
pub mod shapes;

use bevy::prelude::*;
use bevy_steamworks::SteamworksPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            colors::ColorsPlugin,
            main_menu::MainMenuPlugin,
            exit_menu::ExitMenuPlugin,
            game::GamePlugin,
            SteamworksPlugin::init_app(480).unwrap_or_else(error::error_abort),
        ))
        .run();
}
