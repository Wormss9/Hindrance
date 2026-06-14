mod colors;
mod menu;
mod setup;

use bevy::prelude::*;

pub struct PluginsPlugin;

impl Plugin for PluginsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((setup::SetupPlugin, colors::ColorsPlugin, menu::MenuPlugin));
    }
}
