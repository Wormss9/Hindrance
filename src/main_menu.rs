use bevy::{
    asset::RenderAssetUsages,
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    mesh::{Indices, PrimitiveTopology},
    post_process::bloom::Bloom,
    prelude::*,
};

pub fn main_menu(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let purple = materials.add(Color::srgb(0.7, 0.5, 1.0));
    let light_purple = materials.add(Color::srgb(0.9, 0.8, 1.0));
    let dark_purple = materials.add(Color::srgb(0.5, 0.0, 1.0));

    let green = materials.add(Color::srgb(0.5, 1.0, 0.5));
    let light_green = materials.add(Color::srgb(0.8, 1.0, 0.8));
    let dark_green = materials.add(Color::srgb(0.3, 0.8, 0.3));

    let red = materials.add(Color::srgb(1.0, 0.5, 0.5));
    let light_red = materials.add(Color::srgb(1.0, 0.8, 0.8));
    let dark_red = materials.add(Color::srgb(0.8, 0.3, 0.3));

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
            MeshMaterial2d(purple.clone()),
            Transform::from_translation(Vec3::new(0., 100., 0.)),
            Pickable::default(),
        ))
        .observe(update_material_on::<Pointer<Over>>(light_purple.clone()))
        .observe(update_material_on::<Pointer<Out>>(purple.clone()))
        .observe(update_material_on::<Pointer<Press>>(dark_purple.clone()))
        .observe(update_material_on::<Pointer<Release>>(light_purple.clone()));

    commands
        .spawn((
            Mesh2d(meshes.add(RegularPolygon::new(16.0, 3))),
            MeshMaterial2d(green.clone()),
            Pickable::default(),
        ))
        .observe(update_material_on::<Pointer<Over>>(light_green.clone()))
        .observe(update_material_on::<Pointer<Out>>(green.clone()))
        .observe(update_material_on::<Pointer<Press>>(dark_green.clone()))
        .observe(update_material_on::<Pointer<Release>>(light_green.clone()));

    commands
        .spawn((
            Mesh2d(meshes.add(cross_mesh(32.0))),
            MeshMaterial2d(red.clone()),
            Transform {
                translation: Vec3::new(0.0, -100.0, 0.0),
                rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_4),
                ..default()
            },
            Pickable::default(),
        ))
        .observe(update_material_on::<Pointer<Over>>(light_red.clone()))
        .observe(update_material_on::<Pointer<Out>>(red.clone()))
        .observe(update_material_on::<Pointer<Press>>(dark_red.clone()))
        .observe(update_material_on::<Pointer<Release>>(light_red.clone()));
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
        [-width / 8., -width / 8., 0.0], // 0
        [width / 8., -width / 8., 0.0],  // 1
        [width / 8., width / 8., 0.0],   // 2
        [-width / 8., width / 8., 0.0],  // 3
        // top arm
        [-width / 8., width / 8., 0.0], // 4
        [width / 8., width / 8., 0.0],  // 5
        [width / 8., width / 2., 0.0],  // 6
        [-width / 8., width / 2., 0.0], // 7
        // bottom arm
        [-width / 8., -width / 2., 0.0], // 8
        [width / 8., -width / 2., 0.0],  // 9
        [width / 8., -width / 8., 0.0],  // 10
        [-width / 8., -width / 8., 0.0], // 11
        // left arm
        [-width / 2., -width / 8., 0.0], // 12
        [-width / 8., -width / 8., 0.0], // 13
        [-width / 8., width / 8., 0.0],  // 14
        [-width / 2., width / 8., 0.0],  // 15
        // right arm
        [width / 8., -width / 8., 0.0], // 16
        [width / 2., -width / 8., 0.0], // 17
        [width / 2., width / 8., 0.0],  // 18
        [width / 8., width / 8., 0.0],  // 19
    ];

    let indices = vec![
        0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16, 17,
        18, 16, 18, 19,
    ];

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 20]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; 20]);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}
