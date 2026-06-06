use crate::{
    colors::Theme,
    game_logic::{BoardParameters, SQUARE_BOARD, Shape, SquareWall, TriangleWall},
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
        shape: Shape,
    ) -> Self {
        let mesh = match shape {
            Shape::Square => Mesh2d(meshes.add(Rectangle::new(
                SQUARE_BOARD.tile_size,
                SQUARE_BOARD.tile_size,
            ))),
            Shape::Triangle => {
                Mesh2d(meshes.add(RegularPolygon::new(SQUARE_BOARD.tile_size * 3. / 2., 3)))
            }
        };
        let params = BoardParameters::from(shape);
        let transform = params.transform_from_coordinates(x, y);
        Self {
            id: Id(shape.get_id(x, y).expect("Board spawning failed!")),
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
        shape: Shape,
        wall: Wall,
    ) -> Self {
        let params: BoardParameters = shape.into();
        let translation = Vec3::new(
            params.offset_size * (x as f32 - params.mid as f32) + params.offset_size / 2.,
            params.offset_size * (params.mid as f32 - y as f32) - params.offset_size / 2.,
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
                SquareWall::Right => x * 2 + y * 2 * (params.size - 1),
                SquareWall::Down => x * 2 + y * 2 * (params.size - 1) + 1,
            },
            Wall::Triangle(triangle_wall) => todo!(),
        };
        let mesh = Mesh2d(meshes.add(Rectangle::new(
            params.tile_size * 2. + params.gap_size,
            params.gap_size,
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
        shape: Shape,
        x: usize,
        y: usize,
        gap: SquareGapPosition,
        wall_entities: &[Entity],
    ) -> Self {
        let board: BoardParameters = shape.into();

        let mesh = Mesh2d(meshes.add(Rectangle::new(
            board.tile_size / 2.,
            board.offset_size - board.tile_size,
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

        let transform = match gap {
            SquareGapPosition::RU => Transform {
                translation: Vec3::new(
                    board.offset_size * (x as f32 - board.mid as f32) + board.offset_size / 2.,
                    board.offset_size * (board.mid as f32 - y as f32) + board.tile_size / 4.,
                    0.,
                ),
                rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                ..Default::default()
            },
            SquareGapPosition::RD => Transform {
                translation: Vec3::new(
                    board.offset_size * (x as f32 - board.mid as f32) + board.offset_size / 2.,
                    board.offset_size * (board.mid as f32 - y as f32) - board.tile_size / 4.,
                    0.,
                ),
                rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                ..Default::default()
            },
            SquareGapPosition::DL => Transform {
                translation: Vec3::new(
                    board.offset_size * (x as f32 - board.mid as f32) - board.tile_size / 4.,
                    board.offset_size * (board.mid as f32 - y as f32) - board.offset_size / 2.,
                    0.,
                ),
                ..Default::default()
            },
            SquareGapPosition::DR => Transform {
                translation: Vec3::new(
                    board.offset_size * (x as f32 - board.mid as f32) + board.tile_size / 4.,
                    board.offset_size * (board.mid as f32 - y as f32) - board.offset_size / 2.,
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
