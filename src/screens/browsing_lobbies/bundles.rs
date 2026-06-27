use super::components::*;
use crate::{components::*, resources::*};
use bevy::prelude::*;
use gameplay::rules::Shape;

#[derive(Bundle)]
pub struct LobbyIconBundle {
    mesh2d: Mesh2d,
    mesh_material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
    lobby_icon: LobbyIcon,
}

impl LobbyIconBundle {
    pub fn new(shape: Shape, colors: &Colors, meshes: &ButtonMeshes) -> Self {
        let (mesh2d, mesh_material) = Self::mesh_and_color_from_shape(shape, meshes, colors);

        Self {
            mesh2d,
            transform: Transform {
                translation: Vec3::new(-100., 0., 0.),
                ..Default::default()
            },
            mesh_material,
            lobby_icon: LobbyIcon,
        }
    }
    pub fn mesh_and_color_from_shape(
        shape: Shape,
        meshes: &ButtonMeshes,
        colors: &Colors,
    ) -> (Mesh2d, MeshMaterial2d<ColorMaterial>) {
        let (icon_shape, color_id) = match shape {
            Shape::Square => (ButtonShape::Square, ColorId::Foe1),
            Shape::Hexagon => (ButtonShape::Triangle, ColorId::Foe2),
        };
        let mesh = Mesh2d(icon_shape.into_mesh(meshes));
        let color = MeshMaterial2d(
            colors
                .materials
                .get(&(color_id, Variant::Normal))
                .expect("Color not found")
                .clone(),
        );
        (mesh, color)
    }
}

#[derive(Bundle)]
pub struct LobbyPlayersBundle {
    text: Text2d,
    color: TextColor,
    font: TextFont,
    transform: Transform,
    lobby_players: LobbyPlayers,
}

impl LobbyPlayersBundle {
    pub fn new(
        shape: Shape,
        colors: &Colors,
        materials: &Assets<ColorMaterial>,
        players: u8,
        max_players: u8,
        fonts: &Fonts,
    ) -> Self {
        let (text, color) = Self::text_and_color_from_players_and_shape(
            players,
            max_players,
            shape,
            colors,
            materials,
        );
        Self {
            text: Text2d(text),
            color: TextColor(color),
            font: TextFont {
                font_size: FontSize::Px(32.),
                font: FontSource::Handle(fonts.jost_semibold.clone()),
                ..default()
            },
            transform: Transform::default(),
            lobby_players: LobbyPlayers,
        }
    }
    pub fn text_and_color_from_players_and_shape(
        players: u8,
        max_players: u8,
        shape: Shape,
        colors: &Colors,
        materials: &Assets<ColorMaterial>,
    ) -> (String, Color) {
        let color_id = match shape {
            Shape::Square => ColorId::Foe1,
            Shape::Hexagon => ColorId::Foe2,
        };
        let color_handle = colors
            .materials
            .get(&(color_id, Variant::Normal))
            .expect("Color not found");
        let text = format!("{} / {}", players, max_players);
        let color = materials
            .get(color_handle)
            .expect("Color material asset not found")
            .color;
        (text, color)
    }
}
