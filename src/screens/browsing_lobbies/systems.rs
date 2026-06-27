use super::{bundles::*, components::*, resources::*};
use crate::{bundles::*, components::*, observers::*, resources::*};
use bevy::prelude::*;
use gameplay::rules::Shape;
use multiplayer::discovery::lobby::*;

const FIRST_ROW_Y: f32 = 300.0;
const ROW_SPACING: f32 = 72.0;

pub fn setup_browsing_lobbies(mut commands: Commands) {
    let browsing_lobbies_entity = commands
        .spawn((Transform::default(), Visibility::Visible))
        .id();

    commands.insert_resource(BrowsingLobbiesMenuData {
        browsing_lobbies_entity,
    });
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub fn update_browsed_lobbies(
    mut commands: Commands,
    browsing_lobbies_data: Res<BrowsingLobbiesMenuData>,
    discovered_lobbies: Res<DiscoveredLobbies>,
    mut lobby_rows: Query<(Entity, &DiscoveredLobbyRow, &mut Transform)>,
    mut lobby_icons: Query<
        (&mut Mesh2d, &mut MeshMaterial2d<ColorMaterial>),
        (With<LobbyIcon>, Without<LobbyJoin>, Without<LobbyPlayers>),
    >,
    mut lobby_players: Query<
        (&mut Text2d, &mut TextColor),
        (With<LobbyPlayers>, Without<LobbyJoin>, Without<LobbyIcon>),
    >,
    mut lobby_buttons: Query<
        (&mut LobbyJoin, &mut MeshMaterial2d<ColorMaterial>),
        (Without<LobbyIcon>, Without<LobbyPlayers>),
    >,
    meshes: Res<ButtonMeshes>,
    colors: Res<Colors>,
    materials: Res<Assets<ColorMaterial>>,
    fonts: Res<Fonts>,
) {
    let mut lobbies: Vec<DiscoveredLobby> =
        discovered_lobbies.lobbies.clone().into_iter().collect();
    lobbies.sort_by_key(|x| x.address);

    let existing_rows = lobby_rows.iter().len();

    for (index, (lobby, (_, row, mut transform))) in
        lobbies.iter().zip(lobby_rows.iter_mut()).enumerate()
    {
        update_row(
            index,
            lobby,
            row,
            &mut transform,
            &mut lobby_icons,
            &mut lobby_players,
            &mut lobby_buttons,
            &meshes,
            &colors,
            &materials,
        );
    }

    for (index, lobby) in lobbies.iter().enumerate().skip(existing_rows) {
        create_row(
            index,
            lobby,
            &mut commands,
            &browsing_lobbies_data,
            &meshes,
            &colors,
            &materials,
            &fonts,
        );
    }

    for (row, _, _) in lobby_rows.iter().skip(lobbies.len()) {
        commands.entity(row).despawn();
    }
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
fn update_row(
    index: usize,
    lobby: &DiscoveredLobby,
    row: &DiscoveredLobbyRow,
    transform: &mut Transform,
    lobby_icons: &mut Query<
        (&mut Mesh2d, &mut MeshMaterial2d<ColorMaterial>),
        (With<LobbyIcon>, Without<LobbyJoin>, Without<LobbyPlayers>),
    >,
    lobby_players: &mut Query<
        (&mut Text2d, &mut TextColor),
        (With<LobbyPlayers>, Without<LobbyJoin>, Without<LobbyIcon>),
    >,
    lobby_buttons: &mut Query<
        (&mut LobbyJoin, &mut MeshMaterial2d<ColorMaterial>),
        (Without<LobbyIcon>, Without<LobbyPlayers>),
    >,
    meshes: &ButtonMeshes,
    colors: &Colors,
    materials: &Assets<ColorMaterial>,
) {
    transform.translation.y = FIRST_ROW_Y - index as f32 * ROW_SPACING;
    let DiscoveredLobbyRow {
        icon,
        players,
        join_button,
    } = row;
    let (mut icon_mesh, mut icon_color) = lobby_icons.get_mut(*icon).unwrap();
    let (mut lobby_text, mut text_color) = lobby_players.get_mut(*players).unwrap();
    let (mut join, mut join_color) = lobby_buttons.get_mut(*join_button).unwrap();

    let BroadcastLobby {
        shape,
        players,
        max_players,
    } = lobby.lobby;
    let (mesh, mesh_color) = LobbyIconBundle::mesh_and_color_from_shape(shape, meshes, colors);

    let (text, color) = LobbyPlayersBundle::text_and_color_from_players_and_shape(
        players,
        max_players,
        shape,
        colors,
        materials,
    );
    *icon_mesh = mesh;
    *icon_color = mesh_color.clone();
    lobby_text.0 = text;
    text_color.0 = color;
    join.ip = lobby.address;
    *join_color = mesh_color;
}


#[allow(clippy::too_many_arguments)]
fn create_row(
    index: usize,
    lobby: &DiscoveredLobby,
    commands: &mut Commands,
    browsing_lobbies_data: &BrowsingLobbiesMenuData,
    meshes: &ButtonMeshes,
    colors: &Colors,
    materials: &Assets<ColorMaterial>,
    fonts: &Fonts,
) {
    let DiscoveredLobby {
        address,
        lobby,
        last_seen: _,
    } = lobby;
    let BroadcastLobby {
        shape,
        players,
        max_players,
    } = lobby;
    let color_id = match shape {
        Shape::Square => ColorId::Foe1,
        Shape::Hexagon => ColorId::Foe2,
    };
    let row = commands
        .spawn((
            Transform::from_xyz(0.0, FIRST_ROW_Y - index as f32 * ROW_SPACING, 0.0),
            Visibility::Inherited,
        ))
        .id();

    commands
        .entity(browsing_lobbies_data.browsing_lobbies_entity)
        .add_child(row);

    let icon = commands
        .spawn(LobbyIconBundle::new(*shape, colors, meshes))
        .id();

    let players = commands
        .spawn(LobbyPlayersBundle::new(
            *shape,
            colors,
            materials,
            *players,
            *max_players,
            fonts,
        ))
        .id();

    let join_button = commands
        .spawn((
            ButtonBundle::new(
                ButtonShape::Arrow,
                color_id,
                100.0,
                0.0,
                0.0,
                colors,
                meshes,
            ),
            LobbyJoin { ip: *address },
        ))
        .with_button_interaction()
        .observe(|event: On<Pointer<Release>>, joins: Query<&LobbyJoin>| {
            println!(
                "Connecting to: {}:7777",
                joins.get(event.entity).unwrap().ip
            );
        })
        .id();

    commands
        .entity(row)
        .add_children(&[icon, players, join_button]);

    commands.entity(row).insert(DiscoveredLobbyRow {
        icon,
        players,
        join_button,
    });
}

pub fn cleanup_browsing_lobbies(
    mut commands: Commands,
    browsing_lobbies_data: Res<BrowsingLobbiesMenuData>,
) {
    commands
        .entity(browsing_lobbies_data.browsing_lobbies_entity)
        .despawn();
}
