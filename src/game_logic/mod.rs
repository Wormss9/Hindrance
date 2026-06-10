use bevy::prelude::*;
pub mod bundles;
pub mod components;
pub mod enums;
pub mod observers;
pub mod resources;
pub mod systems;

#[derive(States, Component, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Owner {
    None,
    #[default]
    Own,
    Foe1,
    Foe2,
}
