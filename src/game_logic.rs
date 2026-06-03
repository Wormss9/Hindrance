use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileId(pub usize);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SquareGapId {
    pub parent: usize,
    pub location: SquareGapLocation,
    pub wall: Entity,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SquareGapLocation {
    RU,
    RD,
    DL,
    DR,
}
