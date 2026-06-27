pub mod lobby;
mod systems;

use bevy::{prelude::*, time::common_conditions::on_timer};
use gameplay::rules::Shape;
use lobby::HostedLobby;
use std::time::Duration;
use systems::*;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum LobbyState {
    #[default]
    None,
    Hosting,
    Joining,
}

pub fn create_lobby(mut commands: Commands, shape: Shape) {
    commands.insert_resource(HostedLobby::new(shape));
}

pub struct DiscoveryPlugin;

impl Plugin for DiscoveryPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LobbyState>()
            .add_systems(OnEnter(LobbyState::Hosting), insert_lobby_broadcast)
            .add_systems(
                Update,
                broadcast_lobby
                    .after(insert_lobby_broadcast)
                    .run_if(in_state(LobbyState::Hosting))
                    .run_if(on_timer(Duration::from_secs(1))),
            )
            .add_systems(OnEnter(LobbyState::Joining), insert_lobby_discovery)
            .add_systems(
                Update,
                (receive_lobbies, prune_lobbies)
                    .after(insert_lobby_discovery)
                    .run_if(in_state(LobbyState::Joining)),
            )
            .add_systems(
                OnExit(LobbyState::Hosting),
                (remove_lobby_broadcast, remove_lobby),
            )
            .add_systems(OnExit(LobbyState::Joining), remove_lobby_discovery);
    }
}
