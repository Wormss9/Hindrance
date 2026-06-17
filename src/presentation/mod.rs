pub mod bundles;
pub mod components;
pub mod observers;
pub mod resources;
mod screens;
pub mod state;
mod systems;

use crate::presentation::{
    resources::{ButtonMeshes, Colors, Fonts},
    screens::main::MainMenuPlugin,
    state::ScreenState,
    systems::{add_camera, set_window_icon},
};
use bevy::prelude::*;

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
