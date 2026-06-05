use bevy::prelude::*;

use crate::{
    colors::{PointerColorInteraction, Theme},
    main_menu::GameState,
    shapes::{arrow_mesh, cross_mesh},
};

pub struct ExitMenuPlugin;

impl Plugin for ExitMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ExitMenuState>()
            .add_systems(OnEnter(ExitMenuState::Exiting), setup_exit_menu)
            .add_systems(OnExit(ExitMenuState::Exiting), cleanup_exit_menu);
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum ExitMenuState {
    #[default]
    False,
    Exiting,
}

#[derive(Resource)]
struct ExitMenuData {
    exit_menu_entity: Entity,
}

pub fn setup_exit_menu(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    theme: Res<Theme>,
) {
    let exit_menu_entity = commands
        .spawn((Transform::default(), Visibility::Visible))
        .with_children(|parent| {
            parent.spawn((
                Mesh2d(meshes.add(Rectangle::new(2000., 2000.))),
                Transform::from_translation(Vec3::new(0., 0., 1.)),
                MeshMaterial2d(theme.curtain.normal.clone()),
                Pickable::default(),
            ));
            parent
                .spawn((
                    Mesh2d(meshes.add(arrow_mesh(32.0))),
                    Transform::from_translation(Vec3::new(50., 0., 2.)),
                    Pickable::default(),
                ))
                .with_color_set(&theme.own)
                .observe(
                    |_: On<Pointer<Release>>, mut exit_state: ResMut<NextState<ExitMenuState>>| {
                        exit_state.set(ExitMenuState::False);
                    },
                );
            parent
                .spawn((
                    Mesh2d(meshes.add(cross_mesh(32.0))),
                    Transform {
                        translation: Vec3::new(-50., -0., 2.),
                        rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_4),
                        ..default()
                    },
                    Pickable::default(),
                ))
                .with_color_set(&theme.exit)
                .observe(
                    |_: On<Pointer<Release>>,
                     mut exit_state: ResMut<NextState<ExitMenuState>>,
                     mut main_state: ResMut<NextState<GameState>>| {
                        main_state.set(GameState::MainMenu);
                        exit_state.set(ExitMenuState::False);
                    },
                );
        })
        .id();

    commands.insert_resource(ExitMenuData { exit_menu_entity });
}

fn cleanup_exit_menu(mut commands: Commands, menu_data: Res<ExitMenuData>) {
    commands.entity(menu_data.exit_menu_entity).despawn();
}
