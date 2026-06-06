pub mod bundles;
pub mod components;
pub mod observers;
pub mod systems;

use bevy::ecs::resource::Resource;
use bevy::prelude::*;

const SQUARE_SIZE: usize = 9;
const TRIANGLE_SIZE: usize = 9;
const TILE_SIZE: f32 = 60.;
const GAP_SIZE: f32 = 15.;

pub const SQUARE: Shape = Shape::Square;
pub const TRIANGLE: Shape = Shape::Triangle;
pub const SQUARE_BOARD: BoardParameters = BoardParameters {
    size: SQUARE_SIZE,
    mid: SQUARE_SIZE / 2,
    tile_size: TILE_SIZE,
    gap_size: GAP_SIZE,
    offset_size: TILE_SIZE + GAP_SIZE,
    shape: Shape::Square,
};

pub const TRIANGLE_BOARD: BoardParameters = BoardParameters {
    size: TRIANGLE_SIZE,
    mid: TRIANGLE_SIZE / 2,
    tile_size: TILE_SIZE,
    gap_size: GAP_SIZE,
    offset_size: TILE_SIZE + GAP_SIZE,
    shape: Shape::Triangle,
};

#[derive(Clone, Copy)]
pub enum Shape {
    Square,
    Triangle,
}

impl Shape {
    pub fn get_id(self, x: usize, y: usize) -> Option<usize> {
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
}

impl From<Shape> for BoardParameters {
    fn from(value: Shape) -> Self {
        match value {
            Shape::Square => SQUARE_BOARD,
            Shape::Triangle => TRIANGLE_BOARD,
        }
    }
}

pub struct BoardParameters {
    pub size: usize,
    pub mid: usize,
    pub tile_size: f32,
    pub gap_size: f32,
    pub offset_size: f32,
    shape: Shape,
}

impl BoardParameters {
    pub fn transform_from_coordinates(&self, x: usize, y: usize) -> Transform {
        match self.shape {
            Shape::Square => Transform::from_translation(Vec3::new(
                self.offset_size * (x as f32 - self.mid as f32),
                self.offset_size * (self.mid as f32 - y as f32),
                0.,
            )),
            Shape::Triangle => todo!(),
        }
    }
}

#[derive(Resource)]
pub struct Edges {
    size: usize,
    shape: Shape,
    walls: Vec<bool>,
    pub edges: Vec<Vec<usize>>,
}

impl Edges {
    pub fn new(shape: Shape) -> Self {
        let params: BoardParameters = shape.into();
        match shape {
            Shape::Square => Self::square(params.size),
            Shape::Triangle => Self::triangle_hex(Shape::Triangle, params.size),
        }
    }
    fn square(size: usize) -> Self {
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

        let walls = Vec::with_capacity((size - 1) * (size - 1) * 2);

        Self {
            edges,
            size,
            walls,
            shape: Shape::Square,
        }
    }
    fn triangle_hex(shape: Shape, size: usize) -> Self {
        let max = 6 * size * size;
        let mut edges = vec![Vec::with_capacity(3); max];
        for y in 0..size * 2 {
            for x in 0..size * 4 {
                match shape.get_id(x, y) {
                    Some(current) => {
                        if x > 0
                            && let Some(right) = shape.get_id(x - 1, y)
                        {
                            edges[current].push(right);
                        }
                        if x < size * 4 - 1
                            && let Some(left) = shape.get_id(x + 1, y)
                        {
                            edges[current].push(left);
                        }
                        if x % 2 == 0 {
                            if y > 0
                                && let Some(top) =
                                    shape.get_id(if y < size { x - 1 } else { x + 1 }, y - 1)
                            {
                                edges[current].push(top);
                            }
                        } else if y < size * 2
                            && let Some(bottom) =
                                shape.get_id(if y < size { x - 1 } else { x + 1 }, y + 1)
                        {
                            edges[current].push(bottom);
                        }
                    }
                    None => continue,
                }
            }
        }

        let walls = Vec::with_capacity((size - 1) * (size - 1) * 2);

        Self {
            edges,
            size,
            shape: Shape::Triangle,
            walls,
        }
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
