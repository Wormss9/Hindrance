use bevy::prelude::*;

#[derive(Component, Hash, PartialEq, Eq, Clone, Copy)]
pub enum ColorId {
    Own,
    Foe1,
    Foe2,
    Tile,
    ReachableTile,
    Wall,
    Exit,
    Misc,
    Curtain,
}
