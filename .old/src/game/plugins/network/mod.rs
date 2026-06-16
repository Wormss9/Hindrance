mod client;
mod server;

use crate::game::{states::*, systems::start_game};
use bevy::prelude::*;
use client::*;
use server::*;

pub const PORT: u16 = 7777;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(HostingState::Hosting), start_listening)
            .add_systems(OnExit(HostingState::Hosting), stop_listening)
            .add_systems(OnExit(HostingState::False), start_connection)
            .add_systems(
                Update,
                handle_server_messages
                    .after(start_connection)
                    .run_if(in_state(HostingState::Hosting).or(in_state(HostingState::Joining))),
            )
            .add_systems(
                Update,
                (handle_client_messages, server_counters)
                    .after(start_listening)
                    .run_if(in_state(HostingState::Hosting)),
            )
            .add_systems(Update, start_game);
    }
}
