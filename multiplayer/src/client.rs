// use crate::{HostingState, PORT};
// use bevy::prelude::*;
// // use bevy_quinnet::client::{ClientConnectionConfiguration, QuinnetClient, certificate::CertificateVerificationMode, connection::ClientAddrConfiguration};

// pub struct ClientPlugin;

// impl Plugin for ClientPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(
//             OnEnter(HostingState::Hosting),
//             (prepare_client, start_client.after(prepare_client)),
//         );
//         app.add_systems(
//             OnEnter(HostingState::Hosting),
//             (prepare_client, start_client.after(prepare_client)),
//         );
//     }
// }

// pub fn prepare_client(mut commands: Commands) {
//     commands.init_resource::<QuinnetClient>();
// }



// pub fn start_client(mut client: ResMut<QuinnetClient>, steam: Res<Client>) {
//     client
//         .open_connection(ClientConnectionConfiguration {
//             addr_config: ClientAddrConfiguration::from_ips(
//                 std::net::Ipv4Addr::new(127, 0, 0, 1),
//                 PORT,
//                 std::net::Ipv4Addr::UNSPECIFIED,
//                 0,
//             ),
//             cert_mode: CertificateVerificationMode::SkipVerification,
//             defaultables: default(),
//         })
//         .unwrap();

//     let player = steam.user().steam_id().raw();

//     client
//         .connection_mut()
//         .try_send_message::<ClientMessage>(ClientMessage::Join { player });
// }

// pub fn handle_server_messages(
//     steam_client: Res<Client>,
//     mut board_players: ResMut<BoardPlayers>,
//     mut client: ResMut<QuinnetClient>,
//     mut countdown: Query<(&mut Countdown, &mut Visibility), With<Text2d>>,
//     mut lobby_update_writer: MessageWriter<UpdateLobby>,
//     mut start_game_writer: MessageWriter<StartGame>,
// ) {
//     while let Some(message) = client
//         .connection_mut()
//         .try_receive_message::<ServerMessage>()
//     {
//         match message {
//             ServerMessage::GameIsFull => todo!(),
//             ServerMessage::Players { players } => {
//                 *board_players.players = players
//                     .iter()
//                     .map(|(o, s)| (*o, BoardPlayer::from_id(*s, &steam_client)))
//                     .collect();
//                 lobby_update_writer.write(UpdateLobby);
//             }
//             ServerMessage::StartCountdown {} => {
//                 let (mut countdown, mut visibility) =
//                     countdown.single_mut().expect("Missing/multiple counters");
//                 *countdown = Countdown {
//                     timer: Timer::from_seconds(3., TimerMode::Once),
//                 };
//                 *visibility = Visibility::Visible;
//             }
//             ServerMessage::StartGame { shape } => {
//                 start_game_writer.write(StartGame { shape });
//             }
//             ServerMessage::PlayerMoved { owner, id } => todo!(),
//             ServerMessage::WallPlaced { parent, rotation } => todo!(),
//             ServerMessage::BoardShape { shape } => todo!(),
//         }
//     }
// }
