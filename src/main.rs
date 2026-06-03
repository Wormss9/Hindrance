pub mod colors;
mod error;
pub mod game_logic;
mod grid;
mod main_menu;
mod square;
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
            // GameLogicPugin,
            square::SquarePlugin,
            //TrianglePlugin,
            SteamworksPlugin::init_app(480).unwrap_or_else(error::error_abort),
        ))
        .run();
}
