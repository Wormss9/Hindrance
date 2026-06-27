use bevy::{platform::collections::HashSet, prelude::*};
use bitcode::{Decode, Encode};
use gameplay::rules::Shape;
use networking::broadcast::*;
use std::{
    hash::{Hash, Hasher},
    net::IpAddr,
    time::Instant,
};

#[derive(Resource)]
pub struct LobbyDiscovery {
    pub discovery: LanDiscovery,
}

#[derive(Encode, Decode, Clone)]
pub struct BroadcastLobby {
    pub shape: Shape,
    pub players: u8,
    pub max_players: u8,
}

#[derive(Resource)]
pub struct HostedLobby {
    pub shape: Shape,
    pub players: u8,
    pub max_players: u8,
}

#[derive(Clone)]
pub struct DiscoveredLobby {
    pub address: IpAddr,
    pub lobby: BroadcastLobby,
    pub last_seen: Instant,
}

impl PartialEq for DiscoveredLobby {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl Eq for DiscoveredLobby {}

impl Hash for DiscoveredLobby {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state);
    }
}

impl From<(IpAddr, BroadcastLobby)> for DiscoveredLobby {
    fn from(value: (IpAddr, BroadcastLobby)) -> Self {
        let (address, lobby) = value;
        let last_seen = Instant::now();
        Self {
            address,
            lobby,
            last_seen,
        }
    }
}

#[derive(Resource, Default)]
pub struct DiscoveredLobbies {
    pub lobbies: HashSet<DiscoveredLobby>,
}

impl From<&HostedLobby> for BroadcastLobby {
    fn from(value: &HostedLobby) -> Self {
        let &HostedLobby {
            shape,
            players,
            max_players,
        } = value;
        Self {
            shape,
            players,
            max_players,
        }
    }
}

impl HostedLobby {
    pub(crate) fn new(shape: Shape) -> Self {
        Self {
            shape,
            players: 0,
            max_players: shape.max_players(),
        }
    }
}
