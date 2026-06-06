use crate::game_logic::{
    BoardParameters, Edges, Shape,
    components::{Id, Interactable, Own, Pointable, SquareGap, Tile, Wall},
};
use bevy::prelude::*;

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

pub fn show_wall(
    event: On<Pointer<Over>>,
    gap_query: Query<&SquareGap>,
    mut visibility_query: Query<&mut Visibility>,
) {
    let hovered_entity = event.event_target();
    let Ok(gap) = gap_query.get(hovered_entity) else {
        return;
    };
    if let Ok(mut visibility) = visibility_query.get_mut(gap.wall) {
        *visibility = Visibility::Visible;
    }
}

pub fn hide_wall(
    event: On<Pointer<Out>>,
    gap_query: Query<&SquareGap>,
    mut visibility_query: Query<&mut Visibility>,
) {
    let hovered_entity = event.event_target();

    let Ok(gap) = gap_query.get(hovered_entity) else {
        return;
    };

    if let Ok(mut visibility) = visibility_query.get_mut(gap.wall) {
        *visibility = Visibility::Hidden;
    }
}
pub fn move_own(
    event: On<Pointer<Release>>,
    mut own: Query<(&mut Transform, &mut Id), With<Own>>,
    tile: Query<(&Interactable, &Transform, &Id), (With<Tile>, Without<Own>)>,
) {
    let Ok((interactable, target_transform, tile_id)) = tile.get(event.event_target()) else {
        return;
    };
    let Ok((mut own_transform, mut own_id)) = own.single_mut() else {
        return;
    };
    if interactable.0 {
        *own_transform = *target_transform;
        own_id.0 = tile_id.0;
    }
}
pub fn place_wall(
    shape: Shape,
) -> impl FnMut(
    On<Pointer<Release>>,
    Commands,
    Query<&SquareGap>,
    Query<(Entity, &SquareGap)>,
    Query<(Entity, &Id), With<Wall>>,
    ResMut<Edges>,
) {
    move |event: On<Pointer<Release>>,
          commands: Commands,
          this_gap: Query<&SquareGap>,
          gap_query: Query<(Entity, &SquareGap)>,
          wall_query: Query<(Entity, &Id), With<Wall>>,
          edges_query: ResMut<Edges>| {
        let target = event.event_target();
        let this_gap = this_gap.get(target).expect("Failed to get clicked gap!");
        let board: BoardParameters = shape.into();
        let wall = this_gap.parent;
        add_wall(
            wall,
            board.size,
            commands,
            wall_query,
            gap_query,
            edges_query,
        );
    }
}

fn add_wall(
    wall: usize,
    size: usize,
    mut commands: Commands,
    wall_query: Query<(Entity, &Id), With<Wall>>,
    gap_query: Query<(Entity, &SquareGap)>,
    mut edges: ResMut<Edges>,
) {
    let mut walls = Vec::with_capacity(3);
    let wall_size = (size - 1) * 2;
    walls.push(wall);
    if wall.is_multiple_of(2) {
        walls.push(wall + 1);
        if wall >= wall_size {
            walls.push(wall - wall_size)
        };
        walls.push(wall + wall_size);
    } else {
        walls.push(wall - 1);
        if wall % wall_size > 1 {
            walls.push(wall - 2)
        };
        if wall % wall_size < wall_size - 1 {
            walls.push(wall + 2);
        }
    };

    let wall_entities: Vec<Entity> = wall_query
        .iter()
        .filter_map(|(entity, id)| walls.contains(&id.0).then_some(entity))
        .collect();
    let gaps_to_despawn: Vec<Entity> = gap_query
        .iter()
        .filter_map(|(entity, square_gap)| {
            wall_entities.contains(&square_gap.wall).then_some(entity)
        })
        .collect();
    for entity in gaps_to_despawn {
        commands.entity(entity).despawn();
    }

    let tile_base = wall / 2;
    let y = tile_base / (size - 1);
    let x = tile_base % (size - 1);

    let tile_id = x + y * size;
    let tile_bellow_id = tile_id + size;

    if wall.is_multiple_of(2) {
        edges.remove_edge(tile_id, tile_id + 1);
        edges.remove_edge(tile_bellow_id, tile_bellow_id + 1);
    } else {
        edges.remove_edge(tile_id, tile_bellow_id);
        edges.remove_edge(tile_id + 1, tile_bellow_id + 1);
    };
}
