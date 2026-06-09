use crate::{
    colors::Theme,
    game_logic::{
        Edges, WallCount,
        components::{CounterText, Foe, Goal, Id, Interactable, Own, Pointable, Tile},
    },
};
use bevy::prelude::*;

pub fn update_reachable_tiles(
    mut query: Query<
        (
            &Goal,
            &mut MeshMaterial2d<ColorMaterial>,
            &Pointable,
            &Id,
            &mut Interactable,
        ),
        With<Tile>,
    >,
    own_query: Query<&Id, With<Own>>,
    foe_query: Query<&Id, With<Foe>>,
    theme: Res<Theme>,
    edges: Res<Edges>,
) {
    let Ok(own_location) = own_query.single() else {
        return;
    };

    let foe_locations: Vec<usize> = foe_query.iter().map(|id| id.0).collect();

    let reachable_ids = edges.reachable_from(own_location.0, &foe_locations);

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
