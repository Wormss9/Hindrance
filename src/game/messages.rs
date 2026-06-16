use crate::shapes::Shape;
use bevy::prelude::*;

#[derive(Message)]
pub struct UpdateLobby;

#[derive(Message)]
pub struct StartGame {
    pub shape: Shape,
}
