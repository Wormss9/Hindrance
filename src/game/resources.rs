use super::{Owner, components::*, enums::*};
use crate::shapes::{Hexagon, Shape, ShapeTrait, Square};
use bevy::prelude::*;
use bevy::{ecs::resource::Resource, platform::collections::HashMap};
use std::collections::VecDeque;

const SQUARE_SIZE: usize = 9;
const TRIANGLE_SIZE: usize = 4;
const TILE_SIZE: f32 = 60.;
const GAP_SIZE: f32 = 15.;

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

#[derive(Resource, Clone)]
pub struct Edges {
    pub edges: Vec<Vec<usize>>,
    pub goals: HashMap<Owner, Vec<usize>>,
}

impl Edges {
    fn new(board: Board) -> Self {
        match &board.shape {
            Shape::Square(_) => Self::square(&board),
            Shape::Hexagon(hexagon) => Self::triangle(&board, hexagon),
        }
    }
    fn square(board: &Board) -> Self {
        let size = board.size();
        let max = size * size;

        let mut edges = vec![Vec::with_capacity(4); max];
        let mut goals: HashMap<Owner, Vec<usize>> = HashMap::new();

        for i in 0..size {
            for j in 0..size {
                let id = i + j * size;
                goals.entry(board.goal(i, j)).or_default().push(id);

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
        Self { edges, goals }
    }
    fn triangle(board: &Board, shape: &Hexagon) -> Self {
        let size = board.size();
        let max = 6 * size * size;
        let mut edges = vec![Vec::with_capacity(3); max];
        let (x_size, y_size) = board.shape.grid_dimensions();
        let mut goals: HashMap<Owner, Vec<usize>> = HashMap::new();
        for y in 0..size * 2 {
            for x in 0..size * 4 {
                match board.get_id(x, y) {
                    Some(current) => {
                        goals.entry(board.goal(x, y)).or_default().push(current);
                        if x > 0
                            && let Some(right) = board.get_id(x - 1, y)
                        {
                            edges[current].push(right);
                        }
                        if x < x_size - 1
                            && let Some(left) = board.get_id(x + 1, y)
                        {
                            edges[current].push(left);
                        }
                        if !shape.is_triangle_rotated(x, y).unwrap() {
                            if y > 0
                                && let Some(top) = board.get_id(x + 1, y - 1)
                            {
                                edges[current].push(top);
                            }
                        } else if y < y_size
                            && let Some(bottom) = board.get_id(x - 1, y + 1)
                        {
                            edges[current].push(bottom);
                        }
                    }
                    None => continue,
                }
            }
        }
        Self { edges, goals }
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
    pub fn are_goals_reachable(&self, players: Query<(&Owner, &Id), With<Character>>) -> bool {
        for character in players {
            if !self.can_reach(character) {
                return false;
            }
        }
        true
    }
    fn can_reach(&self, character: (&Owner, &Id)) -> bool {
        let mut visited = vec![false; self.edges.len()];
        let mut queue = VecDeque::new();
        let (owner, id) = character;
        let goals = self.goals.get(owner).expect("Goals not found");

        visited[id.0] = true;
        queue.push_back(id.0);

        while let Some(tile) = queue.pop_front() {
            if goals.contains(&tile) {
                return true;
            }

            for &next in self.edges[tile].iter() {
                if !visited[next] {
                    visited[next] = true;
                    queue.push_back(next);
                }
            }
        }

        false
    }
}

#[derive(Resource, Clone)]
pub struct Board {
    pub tile_size: f32,
    pub gap_size: f32,
    pub shape: Shape,
    pub max_walls: usize,
}

impl ShapeTrait for Board {
    fn size(&self) -> usize {
        self.shape.size()
    }

    fn get_id(&self, x: usize, y: usize) -> Option<usize> {
        self.shape.get_id(x, y)
    }

    fn get_local_xy(&self, id: usize) -> Option<(usize, usize)> {
        self.shape.get_local_xy(id)
    }

    fn goal(&self, x: usize, y: usize) -> Owner {
        self.shape.goal(x, y)
    }

    fn grid_dimensions(&self) -> (usize, usize) {
        self.shape.grid_dimensions()
    }

    fn grid_mids(&self) -> (usize, usize) {
        self.shape.grid_mids()
    }

    fn rotate(&self, id: usize, owner: &Owner) -> (usize, Option<usize>) {
        self.shape.rotate(id, owner)
    }
}

impl Board {
    pub fn new_square() -> Board {
        Board {
            tile_size: TILE_SIZE,
            gap_size: GAP_SIZE,
            shape: Shape::Square(Square::from(SQUARE_SIZE)),
            max_walls: 10,
        }
    }
    pub fn new_triangle() -> Board {
        Board {
            tile_size: TILE_SIZE,
            gap_size: GAP_SIZE,
            shape: Shape::Hexagon(Hexagon::from(TRIANGLE_SIZE)),
            max_walls: 7,
        }
    }
    pub fn into_tile_transform(&self, x: usize, y: usize) -> Transform {
        let (mid_x, mid_y) = self.shape.grid_mids();
        match &self.shape {
            Shape::Square(_) => Transform::from_translation(Vec3::new(
                self.get_x_offset() * (x as f32 - mid_x as f32),
                self.get_y_offset() * (mid_y as f32 - y as f32),
                0.,
            )),
            Shape::Hexagon(hexagon) => {
                let points_downwards = hexagon
                    .is_triangle_rotated(x, y)
                    .expect("Triangle out of bounds");
                let angle = match points_downwards {
                    true => 0.,
                    false => std::f32::consts::PI,
                };
                let row_offset = (y as f32 - mid_y as f32) * (self.get_x_offset());
                let rotation_offset = match points_downwards {
                    true => -hexagon.get_triangle_rotation_offset(self.tile_size) / 2.,
                    false => hexagon.get_triangle_rotation_offset(self.tile_size) / 2.,
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
    pub fn get_x_offset(&self) -> f32 {
        let x_offset_correction = self.gap_size * (3.0_f32).sqrt() / 2.0;
        match self.shape {
            Shape::Square(_) => self.tile_size + self.gap_size,
            Shape::Hexagon(_) => self.tile_size / 2. + x_offset_correction,
        }
    }
    pub fn get_y_offset(&self) -> f32 {
        match self.shape {
            Shape::Square(_) => self.tile_size + self.gap_size,
            Shape::Hexagon(_) => (3.0_f32).sqrt() / 2.0 * self.tile_size + self.gap_size * 1.5,
        }
    }

    pub fn get_walls(&self, x: usize, y: usize) -> Vec<Wall> {
        let mut walls = Vec::new();
        match &self.shape {
            Shape::Square(_) => {
                if x < self.size() - 1 && y < self.size() - 1 {
                    walls.push(Wall::Square(SquareWall::Down));
                    walls.push(Wall::Square(SquareWall::Right));
                }
            }
            Shape::Hexagon(hexagon) => {
                let rotated = hexagon.is_triangle_rotated(x, y).unwrap();
                if !rotated && x > 0 && hexagon.get_id(x - 1, y + 1).is_some() {
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

#[derive(Resource)]
pub struct OwnedTimers {
    pub timers: HashMap<Owner, Timer>,
}

#[derive(Resource, Clone)]
pub struct Fonts {
    pub jost_semibold: Handle<Font>,
}

#[derive(Resource)]
pub struct Theme {
    pub own: ColorSet,
    pub foe1: ColorSet,
    pub foe2: ColorSet,
    pub tile: ColorSet,
    pub reachable_tile: ColorSet,
    pub own_tile: ColorSet,
    pub foe1_tile: ColorSet,
    pub foe2_tile: ColorSet,
    pub misc: ColorSet,
    pub wall: ColorSet,
    pub exit: ColorSet,
    pub curtain: ColorSet,
}

impl Theme {
    #[allow(clippy::approx_constant)]
    pub fn new(materials: &mut Mut<Assets<ColorMaterial>>) -> Self {
        let own = Color::srgb(0.518, 0.318, 0.851);
        let foe1 = Color::srgb(0.651, 0.851, 0.318);
        let foe2 = Color::srgb(0.318, 0.651, 0.851);
        let wall: Color = Color::srgb(0.851, 0.518, 0.318);

        Self {
            own: ColorSet::new(materials, own, 0.7),
            foe1: ColorSet::new(materials, foe1, 0.9),
            foe2: ColorSet::new(materials, foe2, 0.9),
            tile: ColorSet::new(materials, Color::srgb(1., 1., 1.), 0.2),
            reachable_tile: ColorSet::new(materials, own, 0.5),
            own_tile: ColorSet::new(materials, own, 0.3),
            foe1_tile: ColorSet::new(materials, foe1, 0.3),
            foe2_tile: ColorSet::new(materials, foe2, 0.3),
            misc: ColorSet::new(materials, Color::srgb(0.8, 0.8, 0.2), 0.7),
            wall: ColorSet::new(materials, wall, 0.7),
            exit: ColorSet::new(materials, Color::srgb(1.0, 0.2, 0.2), 0.7),
            curtain: ColorSet::dark(materials, Color::srgba(0.0, 0.0, 0.0, 0.9)),
        }
    }
}

pub struct ColorSet {
    pub normal: Handle<ColorMaterial>,
    pub light: Handle<ColorMaterial>,
    pub dark: Handle<ColorMaterial>,
}
impl ColorSet {
    pub fn new(materials: &mut Mut<Assets<ColorMaterial>>, base: Color, luminance: f32) -> Self {
        Self {
            normal: materials.add(base.with_luminance(luminance)),
            light: materials.add(base.with_luminance(luminance * 1.5)),
            dark: materials.add(base.with_luminance(luminance / 1.5)),
        }
    }
    pub fn dark(materials: &mut Mut<Assets<ColorMaterial>>, base: Color) -> Self {
        Self {
            normal: materials.add(base),
            light: materials.add(base),
            dark: materials.add(base),
        }
    }
}
