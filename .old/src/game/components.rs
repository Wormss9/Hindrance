use super::{Owner, enums::*, resources::*};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interactable(pub bool);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Pointable {
    pub over: bool,
    pub press: bool,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct Countdown {
    pub timer:Timer
}

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

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Wall {
    Square(SquareWall),
    Triangle(TriangleWall),
}

impl Wall {
    pub fn rotate_wall(&self, owner: &Owner) -> Wall {
        match self {
            Wall::Square(square_wall) => {
                use crate::game::enums::SquareWall::*;
                let w = *square_wall as u8;
                let o = *owner as u8;

                let rotated = (w + o) % 2;

                Wall::Square(match rotated {
                    0 => Down,
                    1 => Right,
                    _ => unreachable!(),
                })
            }
            Wall::Triangle(triangle_wall) => {
                use crate::game::enums::TriangleWall::*;
                let w = *triangle_wall as u8;
                let o = *owner as u8;

                let rotated = (w + o) % 3;

                Wall::Triangle(match rotated {
                    0 => Down,
                    1 => UpRight,
                    2 => DownRight,
                    _ => unreachable!(),
                })
            }
        }
    }
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
pub struct CounterText(pub Owner);

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
