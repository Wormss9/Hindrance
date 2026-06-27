use super::lobby::*;
use bevy::prelude::*;
use networking::broadcast::*;
use std::time::{Duration, Instant};

const DISCOVERY_PORT: u16 = 42_000;
const LOBBY_TTL: Duration = Duration::from_secs(4);

pub(crate) fn insert_lobby_discovery(mut commands: Commands) {
    let discovery = LanDiscovery::new(DISCOVERY_PORT);
    commands.insert_resource(LobbyDiscovery { discovery });
    commands.init_resource::<DiscoveredLobbies>();
}

pub(crate) fn remove_lobby_discovery(mut commands: Commands) {
    commands.remove_resource::<LobbyDiscovery>();
    commands.remove_resource::<DiscoveredLobbies>();
}

pub(crate) fn insert_lobby_broadcast(mut commands: Commands) {
    let discovery = LanBroadcast::new(DISCOVERY_PORT);
    commands.insert_resource(LobbyBroadcast {
        broadcast: discovery,
    });
    commands.init_resource::<DiscoveredLobbies>();
}

pub(crate) fn remove_lobby_broadcast(mut commands: Commands) {
    commands.remove_resource::<LobbyBroadcast>();
}

pub(crate) fn remove_lobby(mut commands: Commands) {
    commands.remove_resource::<HostedLobby>();
}

pub(crate) fn broadcast_lobby(mut discovery: ResMut<LobbyBroadcast>, lobby: Res<HostedLobby>) {
    discovery
        .broadcast
        .broadcast(BroadcastLobby::from(lobby.as_ref()));
}

pub(crate) fn receive_lobbies(
    mut discovery: ResMut<LobbyDiscovery>,
    mut lobbies: ResMut<DiscoveredLobbies>,
) {
    while let Some(bl) = discovery.discovery.try_receive::<BroadcastLobby>() {
        lobbies.lobbies.replace(bl.into());
    }
}

pub(crate) fn prune_lobbies(mut lobbies: ResMut<DiscoveredLobbies>) {
    let now = Instant::now();
    lobbies
        .lobbies
        .retain(|x| now.duration_since(x.last_seen) < LOBBY_TTL);
}
