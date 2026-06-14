use bevy::prelude::*;

use crate::game::resources::Theme;

pub struct ColorsPlugin;

impl Plugin for ColorsPlugin {
    fn build(&self, app: &mut App) {
        app.world_mut().resource_scope(
            |world: &mut World, mut materials: Mut<Assets<ColorMaterial>>| {
                let theme = Theme::new(&mut materials);
                world.insert_resource(theme);
            },
        );
    }
}