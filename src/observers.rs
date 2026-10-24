use super::{components::*, resources::*};
use bevy::prelude::*;

pub trait ButtonInteraction {
    fn with_button_interaction(&mut self) -> &mut Self;
}

impl<'w> ButtonInteraction for EntityCommands<'w> {
    fn with_button_interaction(&mut self) -> &mut Self {
        self.observe(
            move |event: On<PointerOver>,
                  mut self_query: Query<(&mut MeshMaterial2d<ColorMaterial>, &ColorId)>,
                  colors: Res<Colors>| {
                let entity = event.event_target();
                if let Ok((mut material, &color)) = self_query.get_mut(entity) {
                    material.0 = colors
                        .materials
                        .get(&(color, Variant::Light))
                        .expect("Color not found")
                        .clone();
                };
            },
        )
        .observe(
            move |event: On<PointerOut>,
                  mut self_query: Query<(&mut MeshMaterial2d<ColorMaterial>, &ColorId)>,
                  colors: Res<Colors>| {
                let entity = event.event_target();
                if let Ok((mut material, &color)) = self_query.get_mut(entity) {
                    material.0 = colors
                        .materials
                        .get(&(color, Variant::Normal))
                        .expect("Color not found")
                        .clone();
                };
            },
        )
        .observe(
            move |event: On<PointerPress>,
                  mut self_query: Query<(&mut MeshMaterial2d<ColorMaterial>, &ColorId)>,
                  colors: Res<Colors>| {
                let entity = event.event_target();
                if let Ok((mut material, &color)) = self_query.get_mut(entity) {
                    material.0 = colors
                        .materials
                        .get(&(color, Variant::Dark))
                        .expect("Color not found")
                        .clone();
                };
            },
        );
        self
    }
}
