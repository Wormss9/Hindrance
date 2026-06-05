use bevy::prelude::*;

use crate::game_logic::{SquareWall, TriangleWall};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interactable(pub bool);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Pointable {
    pub over: bool,
    pub press: bool,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(pub usize);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridLocation {
    pub x: usize,
    pub y: usize,
}

impl GridLocation {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SquareGapId {
    pub parent: usize,
    pub location: SquareGapLocation,
    pub wall: Entity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SquareGapLocation {
    RU,
    RD,
    DL,
    DR,
}

impl SquareGapId {
    pub fn new(parent: usize, location: SquareGapLocation, wall: Entity) -> Self {
        Self {
            parent,
            location,
            wall,
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Own;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Foe;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Wall {
    Square(SquareWall),
    Triangle(TriangleWall),
}
