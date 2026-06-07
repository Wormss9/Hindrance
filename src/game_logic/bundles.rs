use crate::{
    colors::Theme,
    game_logic::{Board, Shape, SquareWall, TriangleWall},
};

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
    pub fn new(
        meshes: &mut ResMut<'_, Assets<Mesh>>,
        theme: &Theme,
        x: usize,
        y: usize,
        board: Board,
    ) -> Self {
        let mesh = match board.shape {
            Shape::Square => Mesh2d(meshes.add(Rectangle::new(board.tile_size, board.tile_size))),
            Shape::Triangle => {
                Mesh2d(meshes.add(Triangle2d::new(
                    vec2(
                        -board.tile_size / 2.,
                        -(3.0_f32).sqrt() / 4.0 * board.tile_size,
                    ),
                    vec2(
                        board.tile_size / 2.,
                        -(3.0_f32).sqrt() / 4.0 * board.tile_size,
                    ),
                    vec2(
                        0.,
                        (3.0_f32).sqrt() / 4.0 * board.tile_size,
                    ),
                )))
            }
        };
        let transform = board.into_tile_transform(x, y);
        Self {
            id: Id(board.get_tile_id(x, y).expect("Board spawning failed!")),
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
    visibility: Visibility,
    pickable: Pickable,
    transform: Transform,
    id: Id,
    wall: Wall,
}

impl WallBundle {
    pub fn new(
        meshes: &mut ResMut<'_, Assets<Mesh>>,
        theme: &Theme,
        x: usize,
        y: usize,
        board: Board,
        wall: Wall,
    ) -> Self {
        let (mid_x,mid_y) = board.get_mids();
        let translation = Vec3::new(
            board.get_x_offset() * (x as f32 - mid_x as f32) + board.get_x_offset() / 2.,
            board.get_y_offset() * (mid_y as f32 - y as f32) - board.get_y_offset() / 2.,
            0.,
        );
        let bottom_transform = Transform::from_translation(translation);
        let transform = match wall {
            Wall::Square(square_wall) => match square_wall {
                SquareWall::Right => Transform {
                    translation,
                    rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                    ..default()
                },
                SquareWall::Down => bottom_transform,
            },
            Wall::Triangle(triangle_wall) => match triangle_wall {
                TriangleWall::Down => bottom_transform,
                TriangleWall::UpRight => todo!(),
                TriangleWall::DownRight => todo!(),
            },
        };

        let id = match wall {
            Wall::Square(square_wall) => match square_wall {
                SquareWall::Right => x * 2 + y * 2 * (board.size - 1),
                SquareWall::Down => x * 2 + y * 2 * (board.size - 1) + 1,
            },
            Wall::Triangle(triangle_wall) => todo!(),
        };
        let mesh = Mesh2d(meshes.add(Rectangle::new(
            board.tile_size * 2. + board.gap_size,
            board.gap_size,
        )));

        Self {
            mesh,
            mesh_material: MeshMaterial2d(theme.wall.normal.clone()),
            visibility: Visibility::Hidden,
            pickable: Pickable::IGNORE,
            transform,
            id: Id(id),
            wall,
        }
    }
}

#[derive(Bundle)]
pub struct SquareGapBundle {
    gap_id: SquareGap,
    mesh: Mesh2d,
    pickable: Pickable,
    transform: Transform,
}

impl SquareGapBundle {
    pub fn new(
        meshes: &mut ResMut<'_, Assets<Mesh>>,
        board: Board,
        x: usize,
        y: usize,
        gap: SquareGapPosition,
        wall_entities: &[Entity],
    ) -> Self {
        let mesh = Mesh2d(meshes.add(Rectangle::new(
            board.tile_size / 2.,
            board.get_y_offset() - board.tile_size,
        )));

        let gap_id = match gap {
            SquareGapPosition::RU => SquareGap::new(
                2 * x + 2 * (board.size - 1) * (y - 1),
                SquareGapPosition::RU,
                wall_entities[2 * x + 2 * (board.size - 1) * (y - 1)],
            ),
            SquareGapPosition::DR => SquareGap::new(
                2 * x + 2 * (board.size - 1) * y + 1,
                SquareGapPosition::DR,
                wall_entities[2 * x + 2 * (board.size - 1) * y + 1],
            ),
            SquareGapPosition::DL => SquareGap::new(
                2 * x + 2 * (board.size - 1) * y - 1,
                SquareGapPosition::DL,
                wall_entities[2 * x + 2 * (board.size - 1) * y - 1],
            ),
            SquareGapPosition::RD => SquareGap::new(
                2 * x + 2 * (board.size - 1) * y,
                SquareGapPosition::RD,
                wall_entities[2 * x + 2 * (board.size - 1) * y],
            ),
        };
        let (mid_x,mid_y)=board.get_mids();
        let transform = match gap {
            SquareGapPosition::RU => Transform {
                translation: Vec3::new(
                    board.get_x_offset() * (x as f32 - mid_x as f32)
                        + board.get_x_offset() / 2.,
                    board.get_y_offset() * (mid_y as f32 - y as f32)
                        + board.get_y_offset() / 4.,
                    0.,
                ),
                rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                ..Default::default()
            },
            SquareGapPosition::RD => Transform {
                translation: Vec3::new(
                    board.get_x_offset() * (x as f32 - mid_x as f32)
                        + board.get_x_offset() / 2.,
                    board.get_y_offset() * (mid_y as f32 - y as f32)
                        - board.get_y_offset() / 4.,
                    0.,
                ),
                rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                ..Default::default()
            },
            SquareGapPosition::DL => Transform {
                translation: Vec3::new(
                    board.get_x_offset() * (x as f32 - mid_x as f32)
                        - board.get_x_offset() / 4.,
                    board.get_y_offset() * (mid_y as f32 - y as f32)
                        - board.get_y_offset() / 2.,
                    0.,
                ),
                ..Default::default()
            },
            SquareGapPosition::DR => Transform {
                translation: Vec3::new(
                    board.get_x_offset() * (x as f32 - mid_x as f32)
                        + board.get_x_offset() / 4.,
                    board.get_y_offset() * (mid_y as f32 - y as f32)
                        - board.get_y_offset() / 2.,
                    0.,
                ),
                ..Default::default()
            },
        };

        Self {
            mesh,
            transform,
            pickable: Pickable::default(),
            gap_id,
        }
    }
}
