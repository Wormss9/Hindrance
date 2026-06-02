use bevy::{
    asset::RenderAssetUsages,
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    mesh::{Indices, PrimitiveTopology},
    post_process::bloom::Bloom,
    prelude::*,
};

struct ColorPalette {
    normal: Handle<ColorMaterial>,
    light: Handle<ColorMaterial>,
    dark: Handle<ColorMaterial>,
}
impl ColorPalette {
    pub fn new(
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
}

pub fn main_menu(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let purple = ColorPalette::new(
        &mut materials,
        Color::srgb(0.7, 0.5, 1.0),
        Color::srgb(0.9, 0.8, 1.0),
        Color::srgb(0.5, 0.0, 1.0),
    );
    let green = ColorPalette::new(
        &mut materials,
        Color::srgb(0.5, 1.0, 0.5),
        Color::srgb(0.8, 1.0, 0.8),
        Color::srgb(0.3, 0.8, 0.3),
    );
    let yellow = ColorPalette::new(
        &mut materials,
        Color::srgb(0.8, 0.8, 0.0),
        Color::srgb(1.0, 1.0, 0.0),
        Color::srgb(0.5, 0.5, 0.0),
    );
    let red = ColorPalette::new(
        &mut materials,
        Color::srgb(1.0, 0.5, 0.5),
        Color::srgb(1.0, 0.8, 0.8),
        Color::srgb(0.8, 0.3, 0.3),
    );

    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Tonemapping::None,
        Bloom {
            intensity: 0.25,
            ..Default::default()
        }, // 2. Enable bloom for the camera
        DebandDither::Enabled, // Optional: bloom causes gradients which cause banding,
    ));

    commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(32.0, 32.0))),
            MeshMaterial2d(purple.normal.clone()),
            Transform::from_translation(Vec3::new(0., 150., 0.)),
            Pickable::default(),
        ))
        .with_button_colors(&purple);

    commands
        .spawn((
            Mesh2d(meshes.add(RegularPolygon::new(16.0, 3))),
            MeshMaterial2d(green.normal.clone()),
            Transform::from_translation(Vec3::new(0., 50., 0.)),
            Pickable::default(),
        ))
        .with_button_colors(&green);

    commands
        .spawn((
            Mesh2d(meshes.add(arrow_right_mesh(32.0))),
            Transform::from_translation(Vec3::new(0., -50., 0.)),
            MeshMaterial2d(yellow.normal.clone()),
            Pickable::default(),
        ))
        .with_button_colors(&yellow);

    commands
        .spawn((
            Mesh2d(meshes.add(cross_mesh(32.0))),
            MeshMaterial2d(red.normal.clone()),
            Transform {
                translation: Vec3::new(0.0, -150.0, 0.0),
                rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_4),
                ..default()
            },
            Pickable::default(),
        ))
        .with_button_colors(&red)
        .observe(
            |_: On<Pointer<Release>>, mut exit: MessageWriter<AppExit>| {
                exit.write(AppExit::Success);
            },
        );
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

fn cross_mesh(width: f32) -> Mesh {
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
fn arrow_right_mesh(width: f32) -> Mesh {
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

trait ButtonHoverExt {
    fn with_button_colors(self, color: &ColorPalette) -> Self;
}

impl<'w> ButtonHoverExt for EntityCommands<'w> {
    fn with_button_colors(mut self, color: &ColorPalette) -> Self {
        self.observe(update_material_on::<Pointer<Over>>(color.light.clone()))
            .observe(update_material_on::<Pointer<Out>>(color.normal.clone()))
            .observe(update_material_on::<Pointer<Press>>(color.dark.clone()))
            .observe(update_material_on::<Pointer<Release>>(color.light.clone()));

        self
    }
}
