use crate::game::{Owner, components::Countdown, messages::*, resources::*, states::*};
use bevy::prelude::*;
use image::{DynamicImage, RgbaImage};
use strum::IntoEnumIterator;

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<HostingState>()
            .init_resource::<BoardPlayers>()
            .add_message::<UpdateLobby>()
            .add_systems(Update, update_lobby)
            .add_systems(OnEnter(GameState::InLobby), setup_lobby)
            .add_systems(OnExit(GameState::InLobby), cleanup_lobby);
    }
}

#[derive(Resource)]
struct LobbyData {
    lobby_entity: Entity,
}

pub fn setup_lobby(
    mut commands: Commands,
    theme: Res<Theme>,
    fonts: Res<Fonts>,
    materials: Res<Assets<ColorMaterial>>,
) {
    let lobby_entity = commands
        .spawn((Transform::default(), Visibility::Visible))
        .with_children(|parent| {
            for (i, owner) in Owner::iter().enumerate() {
                let color = match owner {
                    Owner::Own => materials.get(&theme.own.normal).unwrap().color,
                    Owner::Foe1 => materials.get(&theme.foe1.normal).unwrap().color,
                    Owner::Foe2 => materials.get(&theme.foe2.normal).unwrap().color,
                    Owner::None => materials.get(&theme.exit.normal).unwrap().color,
                };
                parent.spawn((
                    Text2d::new(""),
                    TextColor(color),
                    TextFont {
                        font_size: 64.,
                        font: fonts.jost_semibold.clone(),
                        ..default()
                    },
                    Transform {
                        translation: Vec3::new(200., -((i as i8) - 1) as f32 * 64., 0.),
                        ..default()
                    },
                    owner,
                ));
                parent.spawn((
                    Sprite { ..default() },
                    Transform {
                        translation: Vec3::new(-200., -((i as i8) - 1) as f32 * 64., 0.),
                        ..default()
                    },
                    Visibility::Hidden,
                    owner,
                ));
            }
            parent.spawn((
                Text2d::new(""),
                Countdown {
                    timer: Timer::from_seconds(3., TimerMode::Once),
                },
                Visibility::Hidden,
            ));
        })
        .id();

    commands.insert_resource(LobbyData { lobby_entity });
}

fn update_lobby(
    mut images: ResMut<Assets<Image>>,
    board_players: Res<BoardPlayers>,
    mut reader: MessageReader<UpdateLobby>,
    mut name_query: Query<(&mut Text2d, &Owner), Without<Sprite>>,
    mut image_query: Query<(&mut Sprite, &mut Visibility, &Owner), Without<Text2d>>,
) {
    for _ in reader.read() {
        for (owner_a, player) in board_players.players.iter() {
            for (mut text, owner_b) in &mut name_query {
                if owner_a == owner_b {
                    text.0 = player.name.clone();
                }
            }

            for (mut sprite, mut visibility, owner_b) in &mut image_query {
                if owner_a == owner_b {
                    let image = RgbaImage::from_raw(64, 64, player.picture.clone()).unwrap();
                    let image = DynamicImage::ImageRgba8(image);
                    let image = Image::from_dynamic(image, true, default());
                    let handle = images.add(image);
                    sprite.image = handle;
                    *visibility = Visibility::Visible;
                }
            }
        }
    }
}

fn cleanup_lobby(mut commands: Commands, lobby_data: Res<LobbyData>) {
    commands.entity(lobby_data.lobby_entity).despawn();
}
