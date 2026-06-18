use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct Fonts {
    pub jost_semibold: Handle<Font>,
}

impl FromWorld for Fonts {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            jost_semibold: asset_server.load("Jost-SemiBold.ttf"),
        }
    }
}
