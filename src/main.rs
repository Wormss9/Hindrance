mod error;
mod grid;
mod main_menu;
mod square;
pub mod user_interface;
pub mod game_logic;

use bevy::prelude::*;
use bevy_steamworks::SteamworksPlugin;
use main_menu::*;

use crate::square::SquarePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            MainMenuPlugin,
            SquarePlugin,
            SteamworksPlugin::init_app(480).unwrap_or_else(error::error_abort),
        ))
        .run();
}

