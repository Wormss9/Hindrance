use crate::shapes::Shape;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub mod bundles;
pub mod components;
pub mod enums;
pub mod messages;
pub mod observers;
pub mod plugins;
mod resources;
pub mod states;
mod systems;

pub use systems as shared1;

pub mod shared2 {
    pub use self::resources::*;
}

#[derive(
    States,
    Component,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    EnumIter,
)]
pub enum Owner {
    Own = 0,
    Foe1 = 1,
    Foe2 = 3,
    #[default]
    None = 4,
}

impl Owner {
    pub fn rotate(&self, owner: &Self, shape: &Shape) -> Self {
        match shape {
            Shape::Square(_) => {
                let s = *self as u8;
                let o = *owner as u8;
                let value = (s + o) % 2;
                match value {
                    0 => Owner::Own,
                    1 => Owner::Foe1,
                    _ => unreachable!(),
                }
            }
            Shape::Hexagon(_) => {
                let s = *self as u8;
                let o = *owner as u8;
                let value = (s + o) % 3;
                match value {
                    0 => Owner::Own,
                    1 => Owner::Foe1,
                    2 => Owner::Foe2,
                    _ => unreachable!(),
                }
            }
        }
    }
}
