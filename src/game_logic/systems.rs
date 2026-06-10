use super::{components::*, resources::*};
use crate::{colors::Theme, game_logic::Owner};

use bevy::prelude::*;

#[allow(clippy::type_complexity)]
pub fn update_reachable_tiles(
    mut query: Query<
        (
            &Owner,
            &mut MeshMaterial2d<ColorMaterial>,
            &Pointable,
            &Id,
            &mut Interactable,
        ),
        With<Tile>,
    >,
    character_query: Query<(&Id, &Owner), With<Character>>,
    theme: Res<Theme>,
    edges: Res<Edges>,
) {
    let mut own_location = None;
    let mut foe_locations = Vec::new();

    for (id, owner) in character_query.iter() {
        match owner {
            Owner::None => {}
            Owner::Own => own_location = Some(id.0),
            Owner::Foe1 | Owner::Foe2 => {
                foe_locations.push(id.0);
            }
        }
    }

    let own_location = own_location.expect("Own character not found");

    let reachable_ids = edges.reachable_from(own_location, &foe_locations);

    for (goal, mut material, pointable, id, mut interactable) in &mut query {
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
            goal.to_color(&theme)
        }
    }
}
pub fn clean_reachable_tiles(
    mut query: Query<(&mut MeshMaterial2d<ColorMaterial>, &mut Interactable), With<Tile>>,
    theme: Res<Theme>,
) {
    for (mut material, mut interactable) in &mut query {
        material.0 = theme.tile.normal.clone();
        interactable.0 = false;
    }
}
pub fn update_counter_text(counter: Res<WallCount>, mut query: Query<(&mut Text2d, &CounterText)>) {
    if !counter.is_changed() {
        return;
    }
    for (mut text, counter_side) in &mut query {
        match counter_side {
            CounterText::OWN => {
                *text = Text2d::new(counter.own.to_string());
            }
            CounterText::FOE => {
                *text = Text2d::new(counter.foe.to_string());
            }
        }
    }
}
