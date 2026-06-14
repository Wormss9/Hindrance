mod error;
mod game;
pub mod meshes;
mod shapes;

use crate::game::plugins;
use bevy::prelude::*;
use bevy_steamworks::SteamworksPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            plugins::PluginsPlugin,
            SteamworksPlugin::init_app(480).unwrap_or_else(error::error_abort),
        ))
        .run();
}
