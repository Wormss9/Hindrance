// mod client;
// mod server;
pub mod discovery;

// // use crate::{systems::start_game};
use bevy::prelude::*;

use crate::discovery::DiscoveryPlugin;
// use bevy_quinnet::{client::QuinnetClientPlugin, server::QuinnetServerPlugin};
// use client::ClientPlugin;
// use server::ServerPlugin;

// pub const PORT: u16 = 7777;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum HostingState {
    #[default]
    None,
    Hosting,
    Joining,
}

pub struct MultiplayerPlugin;

impl Plugin for MultiplayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<HostingState>()
            .add_plugins(DiscoveryPlugin);
    }
}
