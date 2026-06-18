#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod bundles;
pub mod components;
pub mod observers;
pub mod resources;
mod screens;
pub mod state;
mod systems;

use crate::{
    resources::{ButtonMeshes, Colors, Fonts},
    screens::main::MainMenuPlugin,
    state::ScreenState,
    systems::{add_camera, set_window_icon},
};
use bevy::prelude::*;
use bevy_steamworks::SteamworksPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            PresentationPlugin,
            SteamworksPlugin::init_app(480).expect("Steam not running"),
        ))
        .run();
}

pub struct PresentationPlugin;

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ScreenState>()
            .init_resource::<Colors>()
            .init_resource::<ButtonMeshes>()
            .init_resource::<Fonts>()
            .add_plugins(MainMenuPlugin)
            .add_systems(Startup, (set_window_icon, add_camera));
    }
}
