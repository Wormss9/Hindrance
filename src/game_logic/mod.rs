pub mod bundles;
pub mod components;
pub mod observers;
pub mod systems;

use bevy::ecs::resource::Resource;
use bevy::prelude::*;

const SQUARE_SIZE: usize = 9;
const TRIANGLE_SIZE: usize = 4;
const TILE_SIZE: f32 = 60.;
const GAP_SIZE: f32 = 15.;

const SQUARE_BOARD: BoardParameters = BoardParameters {
    size: SQUARE_SIZE,
    tile_size: TILE_SIZE,
    gap_size: GAP_SIZE,
};

const TRIANGLE_BOARD: BoardParameters = BoardParameters {
    size: TRIANGLE_SIZE,
    tile_size: TILE_SIZE,
    gap_size: GAP_SIZE,
};

#[derive(Resource, Clone, Copy)]
pub enum Shape {
    Square,
    Triangle,
}

impl Shape {
    pub fn get_tile_id(self, x: usize, y: usize) -> Option<usize> {
        let params: BoardParameters = self.into();
        let size = params.size;

        match self {
            Shape::Square => {
                if x >= size || y >= size {
                    return None;
                }
                Some(y * size + x)
            }
            Shape::Triangle => {
                if x >= size * 4 || y >= size * 2 || front_skip(x, y, size) || back_skip(x, y, size)
                {
                    return None;
                }

                let raw = y * size * 4 + x;

                let gap = if y < size {
                    (y + 1) * (2 * size - y - 1)
                } else {
                    size * size + (y - size) * (y - size)
                };

                Some(raw - gap)
            }
        }
    }
    pub fn into_tile_transform(self, x: usize, y: usize) -> Transform {
        let board: BoardParameters = self.into();
        let (mid_x, mid_y) = self.get_mids();
        match self {
            Shape::Square => Transform::from_translation(Vec3::new(
                self.get_x_offset() * (x as f32 - mid_x as f32),
                self.get_y_offset() * (mid_y as f32 - y as f32),
                0.,
            )),
            Shape::Triangle => {
                let points_downwards = (self
                    .get_tile_id(x, y)
                    .expect(&format!("Failed to spawn tile {} {}", x, y))
                    + y)
                    .is_multiple_of(2)
                    ^ (y >= board.size);
                let angle = match points_downwards {
                    true => 0.,
                    false => std::f32::consts::PI,
                };
                let (mid_x, mid_y) = self.get_mids();
                let row_offset = (y as f32 - mid_y as f32) * (self.get_x_offset());
                let y_shift = match points_downwards {
                    true => -board.gap_size / 2.,
                    false => 0.,
                };
                //TODO shift rows
                Transform {
                    translation: Vec3 {
                        x: self.get_x_offset() * (x as f32 - mid_x as f32) + row_offset,
                        y: self.get_y_offset() * (mid_y as f32 - y as f32) + y_shift,
                        z: 0.,
                    },
                    rotation: Quat::from_rotation_z(angle),
                    ..Default::default()
                }
            }
        }
    }
    pub fn grid_dimentions(self) -> (usize, usize) {
        let board: BoardParameters = self.into();
        match self {
            Shape::Square => (board.size, board.size),
            Shape::Triangle => (board.size * 4, board.size * 2),
        }
    }
    pub fn get_mids(self) -> (usize, usize) {
        let (x, y) = self.grid_dimentions();
        (x / 2, y / 2)
    }
    pub fn get_x_offset(self) -> f32 {
        let board: BoardParameters = self.into();
        let x_offset_correction = board.gap_size * (3.0_f32).sqrt() / 2.0;
        match self {
            Shape::Square => board.tile_size + board.gap_size,
            Shape::Triangle => board.tile_size / 2. + x_offset_correction,
        }
    }
    pub fn get_y_offset(self) -> f32 {
        let board: BoardParameters = self.into();
        match self {
            Shape::Square => board.tile_size + board.gap_size,
            Shape::Triangle => (3.0_f32).sqrt() / 2.0 * board.tile_size + board.gap_size * 1.5,
        }
    }
}

impl From<Shape> for BoardParameters {
    fn from(value: Shape) -> Self {
        match value {
            Shape::Square => SQUARE_BOARD,
            Shape::Triangle => TRIANGLE_BOARD,
        }
    }
}
impl From<Shape> for Edges {
    fn from(value: Shape) -> Self {
        Edges::new(value)
    }
}

pub struct BoardParameters {
    pub size: usize,
    pub tile_size: f32,
    pub gap_size: f32,
}

#[derive(Resource)]
pub struct Edges {
    pub edges: Vec<Vec<usize>>,
}

impl Edges {
    fn new(shape: Shape) -> Self {
        match shape {
            Shape::Square => Self::square(Shape::Square),
            Shape::Triangle => Self::triangle(Shape::Triangle),
        }
    }
    fn square(shape: Shape) -> Self {
        let size = Into::<BoardParameters>::into(shape).size;
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
    fn triangle(shape: Shape) -> Self {
        let board: BoardParameters = shape.into();
        let size = board.size;
        let max = 6 * size * size;
        let mut edges = vec![Vec::with_capacity(3); max];
        for y in 0..size * 2 {
            for x in 0..size * 4 {
                match shape.get_tile_id(x, y) {
                    Some(current) => {
                        if x > 0
                            && let Some(right) = shape.get_tile_id(x - 1, y)
                        {
                            edges[current].push(right);
                        }
                        if x < size * 4 - 1
                            && let Some(left) = shape.get_tile_id(x + 1, y)
                        {
                            edges[current].push(left);
                        }
                        if x % 2 == 0 {
                            if y > 0
                                && let Some(top) =
                                    shape.get_tile_id(if y < size { x - 1 } else { x + 1 }, y - 1)
                            {
                                edges[current].push(top);
                            }
                        } else if y < size * 2
                            && let Some(bottom) =
                                shape.get_tile_id(if y < size { x - 1 } else { x + 1 }, y + 1)
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

    pub fn reachable_from(&self, own_id: usize, foe_id: usize) -> Vec<usize> {
        let mut reachable = self.edges[own_id].clone();

        if reachable.contains(&foe_id) {
            reachable.retain(|&x| x != foe_id);
            for &location in &self.edges[foe_id] {
                if location != own_id {
                    reachable.push(location);
                }
            }
        }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SquareWall {
    Right,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriangleWall {
    Down,
    UpRight,
    DownRight,
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
