pub mod browsing_lobbies;
pub mod in_lobby;
pub mod main;

use bevy::prelude::*;

use main::MainMenuPlugin;
use browsing_lobbies::BrowsingLobbiesMenuPlugin;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MainMenuPlugin, BrowsingLobbiesMenuPlugin));
    }
}
