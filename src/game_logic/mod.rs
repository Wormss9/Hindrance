pub mod bundles;
pub mod components;
pub mod observers;
pub mod systems;

use bevy::ecs::resource::Resource;
use bevy::prelude::*;
use strum::EnumIter;

use crate::game_logic::components::Wall;

const SQUARE_SIZE: usize = 9;
const TRIANGLE_SIZE: usize = 4;
const TILE_SIZE: f32 = 60.;
const GAP_SIZE: f32 = 15.;

#[derive(Clone, Copy)]
pub enum Shape {
    Square,
    Triangle,
}

impl Board {
    pub const SQUARE_BOARD: Board = Board {
        size: SQUARE_SIZE,
        tile_size: TILE_SIZE,
        gap_size: GAP_SIZE,
        shape: Shape::Square,
        max_walls: 10,
    };
    pub const TRIANGLE_BOARD: Board = Board {
        size: TRIANGLE_SIZE,
        tile_size: TILE_SIZE,
        gap_size: GAP_SIZE,
        shape: Shape::Triangle,
        max_walls: 7,
    };
    pub fn get_tile_id(self, x: usize, y: usize) -> Option<usize> {
        match self.shape {
            Shape::Square => {
                if x >= self.size || y >= self.size {
                    return None;
                }
                Some(y * self.size + x)
            }
            Shape::Triangle => {
                if x >= self.size * 4
                    || y >= self.size * 2
                    || front_skip(x, y, self.size)
                    || back_skip(x, y, self.size)
                {
                    return None;
                }

                let raw = y * self.size * 4 + x;

                let gap = if y < self.size {
                    (y + 1) * (2 * self.size - y - 1)
                } else {
                    self.size * self.size + (y - self.size) * (y - self.size)
                };

                Some(raw - gap)
            }
        }
    }
    pub fn get_triangle_rotation_offset(self) -> f32 {
        (3.0_f32).sqrt() * self.tile_size / 6.
    }
    pub fn into_tile_transform(self, x: usize, y: usize) -> Transform {
        let (mid_x, mid_y) = self.get_mids();
        match self.shape {
            Shape::Square => Transform::from_translation(Vec3::new(
                self.get_x_offset() * (x as f32 - mid_x as f32),
                self.get_y_offset() * (mid_y as f32 - y as f32),
                0.,
            )),
            Shape::Triangle => {
                let points_downwards = self.is_rotated_triangle(x, y);
                let angle = match points_downwards {
                    true => 0.,
                    false => std::f32::consts::PI,
                };
                let (mid_x, mid_y) = self.get_mids();
                let row_offset = (y as f32 - mid_y as f32) * (self.get_x_offset());
                let rotation_offset = match points_downwards {
                    true => -self.get_triangle_rotation_offset() / 2.,
                    false => self.get_triangle_rotation_offset() / 2.,
                };
                let y_shift = match points_downwards {
                    true => 0.,
                    false => self.gap_size / 2.,
                };
                Transform {
                    translation: Vec3 {
                        x: self.get_x_offset() * (x as f32 - mid_x as f32 + 1.) + row_offset,
                        y: self.get_y_offset() * (mid_y as f32 - y as f32 - 0.5)
                            + y_shift
                            + rotation_offset,
                        z: 0.,
                    },
                    rotation: Quat::from_rotation_z(angle),
                    ..Default::default()
                }
            }
        }
    }
    pub fn grid_dimentions(self) -> (usize, usize) {
        match self.shape {
            Shape::Square => (self.size, self.size),
            Shape::Triangle => (self.size * 4, self.size * 2),
        }
    }
    pub fn get_mids(self) -> (usize, usize) {
        let (x, y) = self.grid_dimentions();
        (x / 2, y / 2)
    }
    pub fn get_x_offset(self) -> f32 {
        let x_offset_correction = self.gap_size * (3.0_f32).sqrt() / 2.0;
        match self.shape {
            Shape::Square => self.tile_size + self.gap_size,
            Shape::Triangle => self.tile_size / 2. + x_offset_correction,
        }
    }
    pub fn get_y_offset(self) -> f32 {
        match self.shape {
            Shape::Square => self.tile_size + self.gap_size,
            Shape::Triangle => (3.0_f32).sqrt() / 2.0 * self.tile_size + self.gap_size * 1.5,
        }
    }
    fn is_rotated_triangle(self, x: usize, y: usize) -> bool {
        (self
            .get_tile_id(x, y)
            .unwrap_or_else(|| panic!("Failed to spawn tile {} {}", x, y))
            + y)
            .is_multiple_of(2)
            ^ (y >= self.size)
    }
    pub fn get_walls(self, x: usize, y: usize) -> Vec<Wall> {
        let mut walls = Vec::new();
        match self.shape {
            Shape::Square => {
                if x < self.size - 1 && y < self.size - 1 {
                    walls.push(Wall::Square(SquareWall::Down));
                    walls.push(Wall::Square(SquareWall::Right));
                }
            }
            Shape::Triangle => {
                let rotated = self.is_rotated_triangle(x, y);
                if !rotated && x > 0 && self.get_tile_id(x - 1, y + 1).is_some() {
                    walls.push(Wall::Triangle(TriangleWall::UpRight));
                    walls.push(Wall::Triangle(TriangleWall::DownRight));
                    walls.push(Wall::Triangle(TriangleWall::Down));
                }
            }
        }
        walls
    }
}

impl From<Board> for Edges {
    fn from(value: Board) -> Self {
        Edges::new(value)
    }
}

#[derive(Resource, Clone, Copy)]
pub struct Board {
    pub size: usize,
    pub tile_size: f32,
    pub gap_size: f32,
    pub shape: Shape,
    pub max_walls: usize,
}

#[derive(Resource)]
pub struct Edges {
    pub edges: Vec<Vec<usize>>,
}

impl Edges {
    fn new(board: Board) -> Self {
        match board.shape {
            Shape::Square => Self::square(board),
            Shape::Triangle => Self::triangle(board),
        }
    }
    fn square(board: Board) -> Self {
        let size = board.size;
        let max = size * size;

        let mut edges = vec![Vec::with_capacity(4); max];

        for i in 0..size {
            for j in 0..size {
                let id = i + j * size;
                if j > 0 {
                    edges[id].push(id - size);
                }
                if i > 0 {
                    edges[id].push(id - 1);
                }
                if i + 1 < size {
                    edges[id].push(id + 1);
                }
                if j + 1 < size {
                    edges[id].push(id + size);
                }
            }
        }

        Self { edges }
    }
    fn triangle(board: Board) -> Self {
        let size = board.size;
        let max = 6 * size * size;
        let mut edges = vec![Vec::with_capacity(3); max];
        let (x_size, y_size) = board.grid_dimentions();
        for y in 0..size * 2 {
            for x in 0..size * 4 {
                match board.get_tile_id(x, y) {
                    Some(current) => {
                        if x > 0
                            && let Some(right) = board.get_tile_id(x - 1, y)
                        {
                            edges[current].push(right);
                        }
                        if x < x_size - 1
                            && let Some(left) = board.get_tile_id(x + 1, y)
                        {
                            edges[current].push(left);
                        }
                        if !board.is_rotated_triangle(x, y) {
                            if y > 0
                                && let Some(top) = board.get_tile_id(x + 1, y - 1)
                            {
                                edges[current].push(top);
                            }
                        } else if y < y_size
                            && let Some(bottom) = board.get_tile_id(x - 1, y + 1)
                        {
                            edges[current].push(bottom);
                        }
                    }
                    None => continue,
                }
            }
        }

        Self { edges }
    }

    pub fn reachable_from(&self, own_id: usize, foe_ids: &[usize]) -> Vec<usize> {
        let mut reachable = self.edges[own_id].clone();

        for _ in 0..foe_ids.len() {
            let reacahble_foes: Vec<usize> = reachable
                .iter()
                .copied()
                .filter(|x| foe_ids.contains(x))
                .collect();
            reachable.retain(|x| !foe_ids.contains(x));
            for foe in reacahble_foes {
                reachable.append(&mut self.edges[foe].clone());
            }
        }

        reachable.retain(|x| !foe_ids.contains(x) && *x != own_id);

        reachable
    }
    pub fn remove_edge(&mut self, a: usize, b: usize) {
        self.edges[a].retain(|&x| x != b);
        self.edges[b].retain(|&x| x != a);
    }
}

fn front_skip(x: usize, y: usize, size: usize) -> bool {
    size > y && x < 2 * (size - y) - 1
}

fn back_skip(x: usize, y: usize, size: usize) -> bool {
    y >= size && x > 6 * size - 2 * y - 2
}

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SquareWall {
    Right,
    Down,
}
impl From<SquareWall> for Quat {
    fn from(value: SquareWall) -> Self {
        Quat::from_rotation_z(match value {
            SquareWall::Right => std::f32::consts::FRAC_PI_2,
            SquareWall::Down => 0.,
        })
    }
}

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriangleWall {
    Down,
    UpRight,
    DownRight,
}
impl From<TriangleWall> for Quat {
    fn from(value: TriangleWall) -> Quat {
        Quat::from_rotation_z(match value {
            TriangleWall::Down => 0.,
            TriangleWall::UpRight => -std::f32::consts::FRAC_PI_3,
            TriangleWall::DownRight => std::f32::consts::FRAC_PI_3,
        })
    }
}

#[derive(Resource)]
pub struct WallCount {
    pub own: usize,
    pub foe: usize,
}

impl WallCount {
    pub fn new(max: usize) -> Self {
        Self { own: max, foe: max }
    }
}
