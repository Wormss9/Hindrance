use bevy::prelude::*;

use crate::{colors::Theme, game_logic::{SquareWall, TriangleWall}};

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
pub struct Character;

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
pub struct Gap(pub Entity);


#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Wall {
    Square(SquareWall),
    Triangle(TriangleWall),
}

impl From<Wall> for Quat {
    fn from(value: Wall) -> Quat {
        match value {
            Wall::Square(square_wall) => square_wall.into(),
            Wall::Triangle(triangle_wall) => triangle_wall.into(),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CounterText {
    OWN,
    FOE,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Owner {
    None,
    Own,
    Foe1,
    Foe2,
}

impl Owner {
    pub fn to_color(&self, theme: &Theme) -> Handle<ColorMaterial> {
        match self {
            Owner::None => theme.tile.normal.clone(),
            Owner::Own => theme.own_tile.normal.clone(),
            Owner::Foe1 => theme.foe1_tile.normal.clone(),
            Owner::Foe2 => theme.foe2_tile.normal.clone(),
        }
    }
}
