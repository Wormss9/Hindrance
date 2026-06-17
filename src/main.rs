#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod logic;
mod network;
mod presentation;

use crate::presentation::PresentationPlugin;
use bevy::prelude::*;
use bevy_quinnet::{client::QuinnetClientPlugin, server::QuinnetServerPlugin};
use bevy_steamworks::SteamworksPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            PresentationPlugin,
            SteamworksPlugin::init_app(480).expect("Steam not running"),
            QuinnetClientPlugin::default(),
            QuinnetServerPlugin::default(),
        ))
        .run();
}
