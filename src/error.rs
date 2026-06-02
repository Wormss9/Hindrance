use bevy::prelude::*;
use bevy_steamworks::{SteamAPIInitError, SteamworksPlugin};

pub fn error_abort(e: SteamAPIInitError) -> SteamworksPlugin {
    eprintln!("Steam init failed: {e}");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, move |mut commands: Commands| {
            commands.spawn(Camera2d);
            commands.spawn(Text::new(format!("Steam initialization failed:\n{e}")));
        })
        .run();
    panic!("Steam init failed");
}
