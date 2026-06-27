// use crate::HostingState;
// use bevy::prelude::*;
// use bevy_quinnet::server::{
//     EndpointAddrConfiguration, QuinnetServer, ServerEndpointConfiguration,
//     ServerEndpointConfigurationDefaultables, certificate::CertificateRetrievalMode,
// };

// pub(crate) struct ServerPlugin;

// impl Plugin for ServerPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(
//             OnEnter(HostingState::Hosting),
//             (prepare_server, start_server.after(prepare_server)),
//         );
//     }
// }

// pub(crate) fn prepare_server(mut commands: Commands) {
//     commands.init_resource::<QuinnetServer>();
// }

// pub(crate) fn start_server(mut server: ResMut<QuinnetServer>) {
//     server
//         .start_endpoint(ServerEndpointConfiguration {
//             addr_config: EndpointAddrConfiguration::from_ip(
//                 std::net::Ipv4Addr::UNSPECIFIED,
//                 super::PORT,
//             ),
//             cert_mode: CertificateRetrievalMode::GenerateSelfSigned {
//                 server_hostname: "hindrance_server".to_string(),
//             },
//             defaultables: ServerEndpointConfigurationDefaultables {
//                 ..Default::default()
//             },
//         })
//         .unwrap();
// }

// pub(crate) fn handle_client_messages(
//     mut server: ResMut<QuinnetServer>,
//     mut state: ResMut<LogicState>,
// ) {
//     let endpoint = server.endpoint_mut();
//     for client_id in endpoint.clients() {
//         while let Some(message) =
//             endpoint.try_receive_message_from::<ClientMessage, _>(client_id, 0)
//         {
//             match message {
//                 ClientMessage::Join { player } => join(client_id, player, &mut state, endpoint),
//                 // ClientMessage::Move { id } => move_(client_id, id, &mut state, endpoint),
//                 // ClientMessage::Wall { id, rotation } => {
//                 //     wall(client_id, id, rotation, &mut state, endpoint)
//                 // }
//             };
//         }
//     }
// }
