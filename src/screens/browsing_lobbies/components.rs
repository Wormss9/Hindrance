use bevy::prelude::*;
use std::net::IpAddr;

#[derive(Component)]
pub struct DiscoveredLobbyRow {
    pub icon: Entity,
    pub players: Entity,
    pub join_button: Entity,
}

#[derive(Component)]
pub struct LobbyIcon;

#[derive(Component)]
pub struct LobbyPlayers;

#[derive(Component)]
pub struct LobbyJoin {
    pub ip: IpAddr,
}
