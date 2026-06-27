use crate::{bundles::*, components::*, observers::*, resources::*, state::*};
use bevy::prelude::*;
use gameplay::rules::Shape;
use multiplayer::discovery::{LobbyState, create_lobby};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ScreenState::MainMenu), setup_main_menu)
            .add_systems(OnExit(ScreenState::MainMenu), cleanup_main_menu);
    }
}

#[derive(Resource)]
struct MainMenuData {
    main_menu_entity: Entity,
}

pub fn setup_main_menu(mut commands: Commands, meshes: Res<ButtonMeshes>, colors: Res<Colors>) {
    let main_menu_entity = commands
        .spawn((Transform::default(), Visibility::Visible))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle::new(
                    ButtonShape::Square,
                    ColorId::Own,
                    0.,
                    150.,
                    0.,
                    &colors,
                    &meshes,
                ))
                .with_button_interaction()
                .observe(
                    |_: On<Pointer<Release>>,
                     mut screen_state: ResMut<NextState<ScreenState>>,
                     mut lobby_state: ResMut<NextState<LobbyState>>,
                     commands: Commands| {
                        create_lobby(commands, Shape::Square);
                        screen_state.set(ScreenState::InLobby);
                        lobby_state.set(LobbyState::Hosting);
                    },
                );
            parent
                .spawn(ButtonBundle::new(
                    ButtonShape::Triangle,
                    ColorId::Foe1,
                    0.,
                    50.,
                    0.,
                    &colors,
                    &meshes,
                ))
                .with_button_interaction()
                .observe(
                    |_: On<Pointer<Release>>,
                     mut screen_state: ResMut<NextState<ScreenState>>,
                     mut lobby_state: ResMut<NextState<LobbyState>>,
                     commands: Commands| {
                        create_lobby(commands, Shape::Hexagon);
                        screen_state.set(ScreenState::InLobby);
                        lobby_state.set(LobbyState::Hosting);
                    },
                );
            parent
                .spawn(ButtonBundle::new(
                    ButtonShape::Arrow,
                    ColorId::Misc,
                    0.,
                    -50.,
                    0.,
                    &colors,
                    &meshes,
                ))
                .with_button_interaction()
                .observe(
                    |_: On<Pointer<Release>>,
                     //  mut hosting_state: ResMut<NextState<HostingState>>,
                     mut screen_state: ResMut<NextState<ScreenState>>,
                     mut lobby_state: ResMut<NextState<LobbyState>>| {
                        screen_state.set(ScreenState::BrowsingLobbies);
                        lobby_state.set(LobbyState::Joining);
                    },
                );
            parent
                .spawn(ButtonBundle::new(
                    ButtonShape::Cross,
                    ColorId::Exit,
                    0.,
                    -150.,
                    std::f32::consts::FRAC_PI_4,
                    &colors,
                    &meshes,
                ))
                .with_button_interaction()
                .observe(
                    |_: On<Pointer<Release>>, mut exit: MessageWriter<AppExit>| {
                        exit.write(AppExit::Success);
                    },
                );
        })
        .id();

    commands.insert_resource(MainMenuData { main_menu_entity });
}

fn cleanup_main_menu(mut commands: Commands, menu_data: Res<MainMenuData>) {
    commands.entity(menu_data.main_menu_entity).despawn();
}
