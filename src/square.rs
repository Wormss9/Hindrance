use bevy::prelude::*;

use crate::{main_menu::GameState, user_interface::{ColorPalette, HiglightInteraction}};

pub struct SquarePlugin;

impl Plugin for SquarePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Square), setup_square)
            .add_systems(OnExit(GameState::Square), cleanup_square);
    }
}

#[derive(Resource)]
struct SquareData {
    square_entity: Entity,
}

pub fn setup_square(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let purple = ColorPalette::new(&mut materials, Color::srgb(0.7, 0.5, 1.0), 0.5);
    let green = ColorPalette::new(&mut materials, Color::srgb(0.5, 1.0, 0.5), 0.5);
    let yellow = ColorPalette::new(&mut materials, Color::srgb(0.8, 0.8, 0.0), 0.5);
    let red = ColorPalette::new(&mut materials, Color::srgb(1.0, 0.5, 0.5), 0.5);

    let square_entity = commands
        .spawn((Transform::default(), Visibility::Visible))
        .with_children(|parent| {
            parent
                .spawn((
                    Mesh2d(meshes.add(Rectangle::new(64.0, 64.0))),
                    MeshMaterial2d(purple.get_color()),
                    Transform::from_translation(Vec3::new(0., 0., 0.)),
                    Pickable::default(),
                ))
                .with_button_colors(&purple);
        })
        .id();

    commands.insert_resource(SquareData { square_entity });
}

fn cleanup_square(mut commands: Commands, square_data: Res<SquareData>) {
    commands.entity(square_data.square_entity).despawn();
}
