mod grid;
mod main_menu;

use bevy::prelude::*;
use main_menu::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, main_menu)
        .run();
}