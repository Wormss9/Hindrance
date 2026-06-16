use crate::game::{Owner, components::*, enums::*, resources::*};
use bevy::prelude::*;
use bevy_quinnet::server::{
    EndpointAddrConfiguration, QuinnetServer, ServerEndpointConfiguration,
    ServerEndpointConfigurationDefaultables, certificate::CertificateRetrievalMode,
    endpoint::Endpoint,
};
use strum::IntoEnumIterator;

pub fn start_listening(mut server: ResMut<QuinnetServer>) {
    server
        .start_endpoint(ServerEndpointConfiguration {
            addr_config: EndpointAddrConfiguration::from_ip(
                std::net::Ipv4Addr::UNSPECIFIED,
                super::PORT,
            ),
            cert_mode: CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: "game".to_string(),
            },
            defaultables: ServerEndpointConfigurationDefaultables {
                ..Default::default()
            },
        })
        .unwrap();
}

pub fn stop_listening(mut server: ResMut<QuinnetServer>) {
    let _ = server.stop_endpoint();
}

pub fn handle_client_messages(mut server: ResMut<QuinnetServer>, mut state: ResMut<ServerState>) {
    let endpoint = server.endpoint_mut();
    for client_id in endpoint.clients() {
        while let Some(message) =
            endpoint.try_receive_message_from::<ClientMessage, _>(client_id, 0)
        {
            match message {
                ClientMessage::Join { player } => join(client_id, player, &mut state, endpoint),
                ClientMessage::Move { id } => move_(client_id, id, &mut state, endpoint),
                ClientMessage::Wall { id, rotation } => {
                    wall(client_id, id, rotation, &mut state, endpoint)
                }
            };
        }
    }
}
pub fn server_counters(
    mut server: ResMut<QuinnetServer>,
    mut state: ResMut<ServerState>,
    time: ResMut<Time>,
) {
    let delta = time.delta();
    for timer in state.timers.values_mut() {
        timer.tick(delta);
    }
    state.start_timer.tick(delta);
    println!(
        "Server countdown happening {:?}, paused {}!",
        state.start_timer.duration(),
        state.start_timer.is_paused()
    );
    if state.start_timer.just_finished() {
        println!("Server countdown done!");
        let endpoint = server.endpoint_mut();
        endpoint.try_broadcast_message(ServerMessage::StartGame {
            shape: state.shape.clone(),
        });
    }
}

fn join(client_id: u64, steam_id: u64, state: &mut ServerState, server: &mut Endpoint) {
    let mut empty = vec![];
    for owner in Owner::iter() {
        if owner == Owner::None {
            continue;
        }
        match &state.shape {
            crate::shapes::Shape::Square(_) => {
                if owner == Owner::Foe2 {
                    continue;
                }
            }
            crate::shapes::Shape::Hexagon(_) => {}
        }
        if !state.players.values().any(|(_, x)| x == &owner) {
            empty.push(owner)
        }
    }
    if let Some(&owner) = empty.first() {
        state.players.insert(client_id, (steam_id, owner));

        for (&client_id, (_, owner)) in &state.players {
            let players = state.players.iter().map(|(_, (s, o))| (*o, *s)).collect();
            server
                .send_message(
                    client_id,
                    ServerMessage::Players { players }.to_owner_view(owner, &state.shape),
                )
                .expect("Failed to send");
        }
    } else {
        let _ = server.send_message(client_id, ServerMessage::GameIsFull);
        let _ = server.disconnect_client(client_id);
    }
    match &state.shape {
        crate::shapes::Shape::Square(_) => {
            if state.players.len() == 2 {
                server.try_broadcast_message(ServerMessage::StartCountdown {});
                state.start_timer.unpause();
            }
        }
        crate::shapes::Shape::Hexagon(_) => {
            if state.players.len() == 3 {
                server.try_broadcast_message(ServerMessage::StartCountdown {});
                state.start_timer.unpause();
            }
        }
    }
}
fn move_(client: u64, tile_id: usize, state: &mut ServerState, server: &mut Endpoint) {
    todo!()
}
fn wall(
    client: u64,
    wall_id: usize,
    rotation: Wall,
    state: &mut ServerState,
    server: &mut Endpoint,
) {
    todo!()
}
