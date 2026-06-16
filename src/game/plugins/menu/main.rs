use crate::{
    game::{observers::*, resources::*, states::*},
    meshes::{arrow_mesh, cross_mesh},
};
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu);
    }
}

#[derive(Resource)]
struct MainMenuData {
    main_menu_entity: Entity,
}

pub fn setup_main_menu(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    theme: Res<Theme>,
) {
    let main_menu_entity = commands
        .spawn((Transform::default(), Visibility::Visible))
        .with_children(|parent| {
            parent
                .spawn((
                    Mesh2d(meshes.add(Rectangle::new(32.0, 32.0))),
                    Transform::from_translation(Vec3::new(0., 150., 0.)),
                    Pickable::default(),
                ))
                .with_color_set(&theme.own)
                .observe(
                    |_: On<Pointer<Release>>,
                     mut hosting_state: ResMut<NextState<HostingState>>,
                     mut game_state: ResMut<NextState<GameState>>,
                     mut commands: Commands| {
                        commands.insert_resource(ServerState::new(Board::new_square()));
                        hosting_state.set(HostingState::Hosting);
                        game_state.set(GameState::InLobby);
                    },
                );
            parent
                .spawn((
                    Mesh2d(meshes.add(RegularPolygon::new(16.0, 3))),
                    Transform::from_translation(Vec3::new(0., 50., 0.)),
                    Pickable::default(),
                ))
                .with_color_set(&theme.foe1)
                .observe(
                    |_: On<Pointer<Release>>,
                     mut hosting_state: ResMut<NextState<HostingState>>,
                     mut game_state: ResMut<NextState<GameState>>,
                     mut commands: Commands| {
                        commands.insert_resource(ServerState::new(Board::new_triangle()));
                        hosting_state.set(HostingState::Hosting);
                        game_state.set(GameState::InLobby);
                    },
                );
            parent
                .spawn((
                    Mesh2d(meshes.add(arrow_mesh(32.0))),
                    Transform::from_translation(Vec3::new(0., -50., 0.)),
                    Pickable::default(),
                ))
                .with_color_set(&theme.misc)
                .observe(
                    |_: On<Pointer<Release>>,
                     mut hosting_state: ResMut<NextState<HostingState>>,
                     mut game_state: ResMut<NextState<GameState>>,
                     mut commands: Commands| {
                        commands.insert_resource(Board::new_square());
                        hosting_state.set(HostingState::Joining);
                        game_state.set(GameState::InLobby);
                    },
                );
            parent
                .spawn((
                    Mesh2d(meshes.add(cross_mesh(32.0))),
                    Transform {
                        translation: Vec3::new(0.0, -150.0, 0.0),
                        rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_4),
                        ..default()
                    },
                    Pickable::default(),
                ))
                .with_color_set(&theme.exit)
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
