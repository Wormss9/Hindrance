// use crate::{bundles::*, components::*, observers::*, resources::*, state::*};
// use bevy::prelude::*;
// use multiplayer::{
//     HostingState,
//     discovery::{LobbyState, create_lobby},
// };
// use gameplay::rules::Shape;

// pub struct InLoobyMenuPlugin;

// impl Plugin for InLoobyMenuPlugin {
//     fn build(&self, app: &mut App) {
//         app.init_state::<ScreenState>()
//             .add_systems(OnEnter(ScreenState::InLobby), setup_in_lobby)
//             .add_systems(OnExit(ScreenState::InLobby), cleanup_in_lobby);
//     }
// }

// #[derive(Resource)]
// struct InLobbyData {
//     in_lobby_entity: Entity,
// }

// pub fn setup_in_lobby(mut commands: Commands, meshes: Res<ButtonMeshes>, colors: Res<Colors>) {
//     let in_lobby_entity = commands
//         .spawn((Transform::default(), Visibility::Visible))
//         .with_children(|parent| {
            
//         })
//         .id();

//     commands.insert_resource(InLobbyData { in_lobby_entity });
// }

// fn cleanup_in_lobby(mut commands: Commands, in_lobby_data: Res<InLobbyData>) {
//     commands.entity(in_lobby_data.in_lobby_entity).despawn();
// }
