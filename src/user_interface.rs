use bevy::{asset::RenderAssetUsages, mesh::{Indices, PrimitiveTopology}, prelude::*};



pub struct ColorPalette {
    normal: Handle<ColorMaterial>,
    light: Handle<ColorMaterial>,
    dark: Handle<ColorMaterial>,
}
impl ColorPalette {
    pub fn new(materials: &mut ResMut<Assets<ColorMaterial>>, base: Color, luminance: f32) -> Self {
        Self {
            normal: materials.add(base.with_luminance(luminance)),
            light: materials.add(base.with_luminance(luminance * 1.5)),
            dark: materials.add(base.with_luminance(luminance / 1.5)),
        }
    }
    pub fn new_manual(
        materials: &mut ResMut<Assets<ColorMaterial>>,
        normal: Color,
        light: Color,
        dark: Color,
    ) -> Self {
        Self {
            normal: materials.add(normal),
            light: materials.add(light),
            dark: materials.add(dark),
        }
    }
    pub fn get_color(&self) -> Handle<ColorMaterial> {
        self.normal.clone()
    }
}

pub trait HiglightInteraction {
    fn with_button_colors(&mut self, color: &ColorPalette) -> &mut Self;
}

impl<'w> HiglightInteraction for EntityCommands<'w> {
    fn with_button_colors(&mut self, color: &ColorPalette) -> &mut Self {
        self.observe(update_material_on::<Pointer<Over>>(color.light.clone()))
            .observe(update_material_on::<Pointer<Out>>(color.normal.clone()))
            .observe(update_material_on::<Pointer<Press>>(color.dark.clone()))
            .observe(update_material_on::<Pointer<Release>>(color.light.clone()));

        self
    }
}

fn update_material_on<E: EntityEvent>(
    new_material: Handle<ColorMaterial>,
) -> impl Fn(On<E>, Query<&mut MeshMaterial2d<ColorMaterial>>) {
    move |event, mut query| {
        if let Ok(mut material) = query.get_mut(event.event_target()) {
            material.0 = new_material.clone();
        }
    }
}

pub fn cross_mesh(width: f32) -> Mesh {
    let positions = vec![
        [-width / 8., -width / 2., 0.],
        [width / 8., -width / 2., 0.],
        [width / 8., width / 2., 0.],
        [-width / 8., width / 2., 0.],
        [-width / 2., -width / 8., 0.],
        [width / 2., -width / 8., 0.],
        [width / 2., width / 8., 0.],
        [-width / 2., width / 8., 0.],
    ];

    let indices = vec![0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7];

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 8]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; 8]);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}
pub fn arrow_right_mesh(width: f32) -> Mesh {
    let half_height = width / 4.0;
    let shaft_length = width / 2.0;
    let tip_x = width / 2.0;

    let positions = vec![
        // Shaft rectangle
        [-shaft_length, -half_height, 0.0],
        [0.0, -half_height, 0.0],
        [0.0, half_height, 0.0],
        [-shaft_length, half_height, 0.0],
        // Arrow head
        [0.0, -width / 2.0, 0.0],
        [tip_x, 0.0, 0.0],
        [0.0, width / 2.0, 0.0],
    ];

    let indices = vec![
        // Shaft
        0, 1, 2, 0, 2, 3, // Head
        4, 5, 6,
    ];

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 7]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; 7]);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}
