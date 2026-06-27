#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub(crate) mod bundles;
pub(crate) mod components;
pub(crate) mod observers;
pub(crate) mod resources;
mod screens;
pub(crate) mod state;
mod systems;

use crate::{
    resources::{ButtonMeshes, Colors, Fonts},
    screens::ScreensPlugin,
    state::ScreenState,
    systems::{add_camera, set_window_icon},
};
use bevy::prelude::*;
use multiplayer::MultiplayerPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin, HindrancePlugin))
        .run();
}

pub struct HindrancePlugin;

impl Plugin for HindrancePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ScreenState>()
            .init_resource::<Colors>()
            .init_resource::<ButtonMeshes>()
            .init_resource::<Fonts>()
            .add_systems(Startup, (set_window_icon, add_camera))
            .add_plugins((ScreensPlugin, MultiplayerPlugin));
    }
}
