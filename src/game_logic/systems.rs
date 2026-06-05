use bevy::prelude::*;
use crate::{colors::Theme, game_logic::components::{Foe, Id, Interactable, Own, Pointable, Tile}, grid::Edges};

pub fn update_reachable_tiles(
    mut query: Query<(
        &mut MeshMaterial2d<ColorMaterial>,
        &Pointable,
        &Id,
        &mut Interactable,
    ),With<Tile>>,
    own_query: Query<&Id,With<Own>>,
    foe_query: Query<&Id,With<Foe>>,
    theme: Res<Theme>,
    edges: Res<Edges>,
) {
    let Ok(own_location) = own_query.single() else {
        return;
    };

    let Ok(foe_location) = foe_query.single() else {
        return;
    };

    let reachable_ids = edges.reachable_from(own_location.0, foe_location.0);

    for (mut material, pointable, id, mut interactable) in &mut query {
        let reachable = reachable_ids.contains(&id.0);

        material.0 = if reachable {
            interactable.0 = true;
            if pointable.press {
                theme.reachable_tile.dark.clone()
            } else if pointable.over {
                theme.reachable_tile.light.clone()
            } else {
                theme.reachable_tile.normal.clone()
            }
        } else {
            interactable.0 = false;
            theme.tile.normal.clone()
        }
    }
}

pub fn clean_reachable_tiles(
    mut query: Query<
        (&mut MeshMaterial2d<ColorMaterial>, &mut Interactable),
        With<Tile>,
    >,
    theme: Res<Theme>,
) {
    for (mut material, mut interactable) in &mut query {
        material.0 = theme.tile.normal.clone();
        interactable.0 = false;
    }
}