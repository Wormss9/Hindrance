use bevy::prelude::*;
pub mod bundles;
pub mod components;
pub mod enums;
pub mod observers;
pub mod plugins;
pub mod resources;
pub mod states;
pub mod systems;

#[derive(States, Component, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Owner {
    #[default]
    None,
    Own,
    Foe1,
    Foe2,
}
