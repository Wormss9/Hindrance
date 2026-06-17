use super::{
    components::*,
    resources::*,
};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct ButtonBundle {
    mesh2d: Mesh2d,
    mesh_material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
    pickable: Pickable,
    color_id: ColorId,
}

impl ButtonBundle {
    pub fn new(
        button_shape: ButtonShape,
        color_id: ColorId,
        x: f32,
        y: f32,
        rotation: f32,
        colors: &Colors,
        meshes: &ButtonMeshes,
    ) -> Self {
        Self {
            mesh2d: Mesh2d(button_shape.into_mesh(meshes)),
            transform: Transform {
                translation: Vec3::new(x, y, 0.),
                rotation: Quat::from_rotation_z(rotation),
                ..Default::default()
            },
            pickable: Pickable::default(),
            color_id,
            mesh_material: MeshMaterial2d(
                colors
                    .materials
                    .get(&(color_id, Variant::Normal))
                    .expect("Color nor found")
                    .clone(),
            ),
        }
    }
}
