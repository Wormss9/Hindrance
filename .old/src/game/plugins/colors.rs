use bevy::prelude::*;

use crate::game::resources::Theme;

pub struct ColorsPlugin;

impl Plugin for ColorsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Theme>();
    }
}