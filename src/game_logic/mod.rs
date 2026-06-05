pub mod bundles;
pub mod components;
pub mod systems;

use bevy::prelude::*;
use crate::game_logic::components::{Id, Interactable, Own, Pointable, Tile};

pub trait PointerInteraction {
    fn with_pointer_interaction(&mut self) -> &mut Self;
}

impl<'w> PointerInteraction for EntityCommands<'w> {
    fn with_pointer_interaction(&mut self) -> &mut Self {
        self.observe(
            move |event: On<Pointer<Over>>, mut query: Query<&mut Pointable, With<Tile>>| {
                let entity = event.event_target();
                if let Ok(mut pointable) = query.get_mut(entity) {
                    pointable.over = true;
                };
            },
        )
        .observe(
            move |event: On<Pointer<Out>>, mut query: Query<&mut Pointable, With<Tile>>| {
                let entity = event.event_target();
                if let Ok(mut pointable) = query.get_mut(entity) {
                    pointable.over = false;
                };
            },
        )
        .observe(
            move |event: On<Pointer<Press>>, mut query: Query<&mut Pointable, With<Tile>>| {
                let entity = event.event_target();
                if let Ok(mut pointable) = query.get_mut(entity) {
                    pointable.press = true;
                };
            },
        )
        .observe(
            move |event: On<Pointer<Release>>, mut query: Query<&mut Pointable, With<Tile>>| {
                let entity = event.event_target();
                if let Ok(mut pointable) = query.get_mut(entity) {
                    pointable.press = false;
                };
            },
        );

        self
    }
}

pub trait OwnMovement {
    fn with_move_own(&mut self) -> &mut Self;
}

#[allow(clippy::type_complexity)]
impl<'w> OwnMovement for EntityCommands<'w> {
    fn with_move_own(&mut self) -> &mut Self {
        self.observe(            move |event: On<Pointer<Release>>,
                  mut own: Query<(&mut Transform, &mut Id), With<Own>>,
                  tile: Query<(&Interactable, &Transform, &Id),(With<Tile>,Without<Own>)>| {
                let Ok((interactable, target_transform, tile_id)) = tile.get(event.event_target())
                else {
                    return;
                };
                let Ok((mut own_transform, mut own_id)) = own.single_mut() else {
                    return;
                };
                if interactable.0 {
                    *own_transform = *target_transform;
                    own_id.0 = tile_id.0;
                }
            },
        );

        self
    }
}
