use crate::colors::Theme;

use super::components::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct TileBundle {
    id: Id,
    interactable: Interactable,
    mesh: Mesh2d,
    mesh_material: MeshMaterial2d<ColorMaterial>,
    pickable: Pickable,
    pointable: Pointable,
    tile: Tile,
    transform: Transform,
}

impl TileBundle {
    pub fn new(mesh: Mesh2d, theme: &Theme, transform: Transform, id: usize) -> Self {
        Self {
            id: Id(id),
            interactable: Interactable(false),
            mesh,
            mesh_material: MeshMaterial2d(theme.tile.normal.clone()),
            pickable: Pickable::default(),
            pointable: Pointable::default(),
            tile: Tile,
            transform,
        }
    }
}

#[derive(Bundle)]
pub struct WallBundle {
    mesh: Mesh2d,
    mesh_material: MeshMaterial2d<ColorMaterial>,
    visibility:Visibility,
    pickable: Pickable,
    transform: Transform,
}

impl WallBundle {
    pub fn new(mesh: Mesh2d, theme: &Theme, transform: Transform) -> Self {
        Self {
            mesh,
            mesh_material: MeshMaterial2d(theme.wall.normal.clone()),
            visibility: Visibility::Hidden,
            pickable: Pickable::IGNORE,
            transform,
        }
    }
}
