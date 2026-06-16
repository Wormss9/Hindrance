use super::{Owner, components::*};
use crate::shapes::{Shape, ShapeTrait};
use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Move { id: usize },
    Wall { id: usize, rotation: Wall },
    Join { player: u64 },
}

#[derive(Message, Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    Players { players: HashMap<Owner, u64> },
    StartCountdown {},
    StartGame { shape: Shape },
    PlayerMoved { owner: Owner, id: usize },
    WallPlaced { parent: usize, rotation: Wall },
    BoardShape { shape: Shape },
    GameIsFull,
}

impl ServerMessage {
    pub fn to_owner_view(self, owner: &Owner, shape: &Shape) -> Self {
        match self {
            ServerMessage::WallPlaced { parent, rotation } => {
                let rotation = rotation.rotate_wall(&owner);
                let (_, parent) = shape.rotate_tile(parent, &owner);
                let parent = parent.unwrap();
                ServerMessage::WallPlaced { parent, rotation }
            }
            ServerMessage::PlayerMoved {
                owner: msg_owner,
                id,
            } => {
                let owner = msg_owner.rotate(owner, shape);
                let (id, _) = shape.rotate_tile(id, &owner);

                ServerMessage::PlayerMoved { owner, id }
            }

            ServerMessage::BoardShape { shape } => ServerMessage::BoardShape { shape },

            ServerMessage::Players { players } => {
                let players = players
                    .iter()
                    .map(|(o, v)| (o.rotate(owner, shape), *v))
                    .collect();
                ServerMessage::Players { players }
            }

            ServerMessage::GameIsFull => ServerMessage::GameIsFull,
            ServerMessage::StartCountdown {} => ServerMessage::StartCountdown {},
            ServerMessage::StartGame { shape } => ServerMessage::StartGame { shape },
        }
    }
}

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SquareWall {
    Right = 0,
    Down = 1,
}
impl From<SquareWall> for Quat {
    fn from(value: SquareWall) -> Self {
        Quat::from_rotation_z(match value {
            SquareWall::Right => std::f32::consts::FRAC_PI_2,
            SquareWall::Down => 0.,
        })
    }
}

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TriangleWall {
    Down = 0,
    UpRight = 1,
    DownRight = 2,
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
