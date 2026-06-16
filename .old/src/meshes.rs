use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

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

    make_mesh(positions, indices)
}

pub fn arrow_mesh(width: f32) -> Mesh {
    let half_height = width / 4.0;
    let shaft_length = width / 2.0;
    let tip_x = width / 2.0;

    let positions = vec![
        [-shaft_length, -half_height, 0.0],
        [0.0, -half_height, 0.0],
        [0.0, half_height, 0.0],
        [-shaft_length, half_height, 0.0],
        [0.0, -width / 2.0, 0.0],
        [tip_x, 0.0, 0.0],
        [0.0, width / 2.0, 0.0],
    ];

    let indices = vec![0, 1, 2, 0, 2, 3, 4, 5, 6];

    make_mesh(positions, indices)
}

fn make_mesh(positions: Vec<[f32; 3]>, indices: Vec<u32>) -> Mesh {
    let vertex_count = positions.len();

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; vertex_count]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; vertex_count]);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}
