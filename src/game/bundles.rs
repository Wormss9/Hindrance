use super::{Owner, components::*, resources::*};
use crate::shapes::{Shape, ShapeTrait};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct TileBundle {
    goal: Owner,
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
        board: &Board,
        goal: Owner,
    ) -> Self {
        let mesh = match board.shape {
            Shape::Square(_) => {
                Mesh2d(meshes.add(Rectangle::new(board.tile_size, board.tile_size)))
            }
            Shape::Hexagon(_) => {
                Mesh2d(meshes.add(RegularPolygon::new(board.tile_size / (3.0_f32).sqrt(), 3)))
            }
        };
        let transform = board.into_tile_transform(x, y);
        let color = goal.to_color(theme);
        Self {
            goal,
            id: Id(board.get_id(x, y).expect("Board spawning failed!")),
            interactable: Interactable(false),
            mesh,
            mesh_material: MeshMaterial2d(color),
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
    wall_id: Id,
    wall: Wall,
}

impl WallBundle {
    pub fn new(
        meshes: &mut ResMut<'_, Assets<Mesh>>,
        theme: &Theme,
        x: usize,
        y: usize,
        board: &Board,
        &wall: &Wall,
    ) -> Self {
        let (mid_x, mid_y) = board.shape.grid_mids();
        let transform = Transform {
            translation: match &board.shape {
                Shape::Square(_) => Vec3::new(
                    board.get_x_offset() * (x as f32 - mid_x as f32) + board.get_x_offset() / 2.,
                    board.get_y_offset() * (mid_y as f32 - y as f32) - board.get_y_offset() / 2.,
                    0.,
                ),
                Shape::Hexagon(hexagon) => {
                    board.into_tile_transform(x, y).translation
                        + Vec3 {
                            x: 0.,
                            y: -board.tile_size * (3.0_f32).sqrt() / 3.0
                                - hexagon.get_triangle_rotation_offset(board.tile_size) / 2.
                                - board.gap_size / 2.,
                            z: 0.,
                        }
                }
            },
            rotation: wall.into(),
            ..default()
        };
        let id = board.get_id(x, y).expect("Failed to spawn wall");
        let mesh = Mesh2d(meshes.add(Rectangle::new(
            board.tile_size * 2.
                + match board.shape {
                    Shape::Square(_) => board.gap_size,
                    Shape::Hexagon(_) => board.gap_size * (3.0_f32).sqrt(),
                },
            board.gap_size,
        )));

        Self {
            mesh,
            mesh_material: MeshMaterial2d(theme.wall.normal.clone()),
            visibility: Visibility::Hidden,
            pickable: Pickable::IGNORE,
            transform,
            wall_id: Id(id),
            wall,
        }
    }
}

#[derive(Bundle)]
pub struct GapBundle {
    gap: Gap,
    id: Id,
    location: GridLocation,
    mesh: Mesh2d,
    pickable: Pickable,
    transform: Transform,
    wall: Wall,
}

impl GapBundle {
    pub fn new(
        meshes: &mut ResMut<'_, Assets<Mesh>>,
        board: &Board,
        x: usize,
        y: usize,
        wall_entity: Entity,
        wall: Wall,
        upper: bool,
    ) -> Self {
        let mesh = Mesh2d(meshes.add(Rectangle::new(board.tile_size / 2., board.gap_size)));
        let rotation: Quat = wall.into();
        let gap_offset = match wall {
            Wall::Square(_) => board.gap_size / 2.,
            Wall::Triangle(_) => board.get_x_offset() - board.tile_size / 2.,
        };
        let translation = rotation.mul_vec3(Vec3 {
            x: (gap_offset + board.tile_size / 4.0) * if upper { -1.0 } else { 1.0 },
            y: 0.,
            z: 0.,
        });

        let (mid_x, mid_y) = board.shape.grid_mids();
        let transform = Transform {
            translation: translation
                + match &board.shape {
                    Shape::Square(_) => Vec3::new(
                        board.get_x_offset() * (x as f32 - mid_x as f32)
                            + board.get_x_offset() / 2.,
                        board.get_y_offset() * (mid_y as f32 - y as f32)
                            - board.get_y_offset() / 2.,
                        0.,
                    ),
                    Shape::Hexagon(hexagon) => {
                        board.into_tile_transform(x, y).translation
                            + Vec3 {
                                x: 0.,
                                y: -board.tile_size * (3.0_f32).sqrt() / 3.0
                                    - hexagon.get_triangle_rotation_offset(board.tile_size) / 2.
                                    - board.gap_size / 2.,
                                z: 0.,
                            }
                    }
                },
            rotation: wall.into(),
            ..default()
        };

        Self {
            gap: Gap(wall_entity),
            id: Id(board.get_id(x, y).expect("Failed to spawn gap")),
            location: GridLocation { x, y },
            mesh,
            pickable: Pickable::default(),
            transform,
            wall,
        }
    }
}
