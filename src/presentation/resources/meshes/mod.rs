mod primitives;

use bevy::{mesh::Mesh, prelude::*};
use primitives::*;

const BUTTON_SIZE: f32 = 32.;

pub enum ButtonShape {
    Square,
    Triangle,
    Arrow,
    Cross,
}

impl ButtonShape {
    pub fn into_mesh(&self, meshes: &ButtonMeshes) -> Handle<Mesh> {
        use ButtonShape::*;
        match self {
            Square => &meshes.square,
            Triangle => &meshes.triangle,
            Arrow => &meshes.arrow,
            Cross => &meshes.cross,
        }
        .clone()
    }
}

#[derive(Resource)]
pub struct ButtonMeshes {
    pub square: Handle<Mesh>,
    pub triangle: Handle<Mesh>,
    pub arrow: Handle<Mesh>,
    pub cross: Handle<Mesh>,
}

impl FromWorld for ButtonMeshes {
    fn from_world(world: &mut World) -> Self {
        let mut assets = world.resource_mut::<Assets<Mesh>>();
        Self {
            square: assets.add(Rectangle::from_length(BUTTON_SIZE)),
            triangle: assets.add(RegularPolygon::new(BUTTON_SIZE / 2., 3)),
            arrow: assets.add(Arrow::new(BUTTON_SIZE)),
            cross: assets.add(Cross::new(BUTTON_SIZE)),
        }
    }
}
