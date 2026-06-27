mod bundles;
mod components;
mod resources;
mod systems;

use crate::state::*;
use bevy::prelude::*;
use systems::*;

pub struct BrowsingLobbiesMenuPlugin;

impl Plugin for BrowsingLobbiesMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(ScreenState::BrowsingLobbies),
            setup_browsing_lobbies,
        )
        .add_systems(
            OnExit(ScreenState::BrowsingLobbies),
            cleanup_browsing_lobbies,
        )
        .add_systems(
            Update,
            update_browsed_lobbies.run_if(in_state(ScreenState::BrowsingLobbies)),
        );
    }
}
