use crate::shapes::{Shape, ShapeTrait};

use super::{Owner, components::*, enums::*, resources::*};
use bevy::prelude::*;
use strum::IntoEnumIterator;

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

#[allow(clippy::type_complexity)]
pub fn move_own(
    event: On<Pointer<Release>>,
    own_querry: Query<((&mut Transform, &mut Id), &Owner), With<Character>>,
    tile: Query<(&Interactable, &Transform, &Id), (With<Tile>, Without<Character>)>,
) {
    if event.button != PointerButton::Primary {
        return;
    }
    let mut own = None;

    let (interactable, target_transform, tile_id) = tile
        .get(event.event_target())
        .expect("Target tile not found");
    for (own_p, owner) in own_querry {
        if owner == &Owner::Own {
            own = Some(own_p)
        }
    }
    let (mut own_transform, mut own_id) = own.expect("Failed to find own character");
    if interactable.0 {
        *own_transform = *target_transform;
        own_id.0 = tile_id.0;
    }
}
pub fn show_wall(
    event: On<Pointer<Over>>,
    gap_query: Query<&Gap>,
    mut visibility_query: Query<&mut Visibility>,
) {
    let hovered_entity = event.event_target();
    let Ok(gap) = gap_query.get(hovered_entity) else {
        return;
    };
    if let Ok(mut visibility) = visibility_query.get_mut(gap.0) {
        *visibility = Visibility::Visible;
    }
}

pub fn hide_wall(
    event: On<Pointer<Out>>,
    gap_query: Query<&Gap>,
    mut visibility_query: Query<&mut Visibility>,
) {
    let hovered_entity = event.event_target();

    let Ok(gap) = gap_query.get(hovered_entity) else {
        return;
    };

    if let Ok(mut visibility) = visibility_query.get_mut(gap.0) {
        *visibility = Visibility::Hidden;
    }
}

#[allow(clippy::too_many_arguments)]
pub fn place_wall(
    event: On<Pointer<Release>>,
    commands: Commands,
    players: Query<(&Owner, &Id), With<Character>>,
    target_query: Query<(&Id, &GridLocation, &Wall), With<Gap>>,
    wall_query: Query<(Entity, &Id, &Wall), With<Wall>>,
    gap_query: Query<(Entity, &Gap)>,
    wall_count: ResMut<WallCount>,
    edges: ResMut<Edges>,
    board: Res<Board>,
) {
    let target = event.event_target();
    if event.button != PointerButton::Secondary {
        return;
    }
    let (id, location, rotation) = target_query
        .get(target)
        .expect("Failed to get clicked gap!");
    if add_wall(
        commands, *id, *location, *rotation, wall_query, gap_query, players, wall_count, edges,
        board,
    ) {
        // Next turn
    };
}
#[allow(clippy::too_many_arguments)]
fn add_wall(
    mut commands: Commands,
    id: Id,
    location: GridLocation,
    rotation: Wall,
    wall_query: Query<(Entity, &Id, &Wall), With<Wall>>,
    gap_query: Query<(Entity, &Gap)>,
    players: Query<(&Owner, &Id), With<Character>>,
    mut wall_count: ResMut<WallCount>,
    mut edges: ResMut<Edges>,
    board: Res<Board>,
) -> bool {
    if wall_count.counts.get(&Owner::Own) == Some(&0) {
        return false;
    }
    let mut walls = Vec::new();
    let GridLocation { x, y } = location;
    let mut new_edges = edges.clone();

    match rotation {
        Wall::Square(square_wall) => match square_wall {
            SquareWall::Right => {
                new_edges.remove_edge(board.get_id(x, y).unwrap(), board.get_id(x + 1, y).unwrap());
                new_edges.remove_edge(
                    board.get_id(x, y + 1).unwrap(),
                    board.get_id(x + 1, y + 1).unwrap(),
                );
            }
            SquareWall::Down => {
                new_edges.remove_edge(board.get_id(x, y).unwrap(), board.get_id(x, y + 1).unwrap());
                new_edges.remove_edge(
                    board.get_id(x + 1, y).unwrap(),
                    board.get_id(x + 1, y + 1).unwrap(),
                );
            }
        },
        Wall::Triangle(triangle_wall) => match triangle_wall {
            TriangleWall::Down => {
                new_edges.remove_edge(
                    board.get_id(x - 1, y).unwrap(),
                    board.get_id(x - 2, y + 1).unwrap(),
                );
                new_edges.remove_edge(
                    board.get_id(x + 1, y).unwrap(),
                    board.get_id(x, y + 1).unwrap(),
                );
            }
            TriangleWall::UpRight => {
                new_edges.remove_edge(board.get_id(x, y).unwrap(), board.get_id(x - 1, y).unwrap());
                new_edges.remove_edge(
                    board.get_id(x, y + 1).unwrap(),
                    board.get_id(x - 1, y + 1).unwrap(),
                );
            }
            TriangleWall::DownRight => {
                new_edges.remove_edge(board.get_id(x, y).unwrap(), board.get_id(x + 1, y).unwrap());
                new_edges.remove_edge(
                    board.get_id(x - 1, y + 1).unwrap(),
                    board.get_id(x - 2, y + 1).unwrap(),
                );
            }
        },
    }

    if !new_edges.are_goals_reachable(players) {
        return false;
    }
    *edges = new_edges;

    match board.shape {
        Shape::Square(_) => {
            for location in SquareWall::iter() {
                walls.push((id, Wall::Square(location)))
            }
        }
        Shape::Hexagon(_) => {
            for location in TriangleWall::iter() {
                walls.push((id, Wall::Triangle(location)))
            }
        }
    }
    match rotation {
        Wall::Square(square_wall) => match square_wall {
            SquareWall::Right => {
                if y > 0
                    && let Some(id) = board.get_id(x, y - 1)
                {
                    walls.push((Id(id), Wall::Square(square_wall)))
                }
                if let Some(id) = board.get_id(x, y + 1) {
                    walls.push((Id(id), Wall::Square(square_wall)))
                }
            }
            SquareWall::Down => {
                if x > 0
                    && let Some(id) = board.get_id(x - 1, y)
                {
                    walls.push((Id(id), Wall::Square(square_wall)))
                }
                if let Some(id) = board.get_id(x + 1, y) {
                    walls.push((Id(id), Wall::Square(square_wall)))
                }
            }
        },
        Wall::Triangle(triangle_wall) => match triangle_wall {
            TriangleWall::Down => {
                if x > 1
                    && let Some(id) = board.get_id(x - 2, y)
                {
                    walls.push((Id(id), Wall::Triangle(triangle_wall)))
                }
                if let Some(id) = board.get_id(x + 2, y) {
                    walls.push((Id(id), Wall::Triangle(triangle_wall)))
                }
            }
            TriangleWall::UpRight => {
                if y > 0
                    && x > 0
                    && let Some(id) = board.get_id(x, y - 1)
                {
                    walls.push((Id(id), Wall::Triangle(triangle_wall)))
                }
                if let Some(id) = board.get_id(x, y + 1) {
                    walls.push((Id(id), Wall::Triangle(triangle_wall)))
                }
            }
            TriangleWall::DownRight => {
                if y > 0
                    && let Some(id) = board.get_id(x + 2, y - 1)
                {
                    walls.push((Id(id), Wall::Triangle(triangle_wall)))
                }
                if x > 1
                    && let Some(id) = board.get_id(x - 2, y + 1)
                {
                    walls.push((Id(id), Wall::Triangle(triangle_wall)))
                }
            }
        },
    }

    let wall_entities: Vec<Entity> = wall_query
        .iter()
        .filter_map(|(entity, id, wall)| walls.contains(&(*id, *wall)).then_some(entity))
        .collect();
    let gaps_to_despawn: Vec<Entity> = gap_query
        .iter()
        .filter_map(|(entity, gap)| wall_entities.contains(&gap.0).then_some(entity))
        .collect();
    for entity in gaps_to_despawn {
        commands.entity(entity).despawn();
    }
    *wall_count.counts.get_mut(&Owner::Own).unwrap() -= 1;
    true
}

pub trait PointerColorInteraction {
    fn with_color_set(&mut self, color: &ColorSet) -> &mut Self;
}

impl<'w> PointerColorInteraction for EntityCommands<'w> {
    fn with_color_set(&mut self, color: &ColorSet) -> &mut Self {
        self.observe(update_material_on::<Pointer<Over>>(color.light.clone()))
            .observe(update_material_on::<Pointer<Out>>(color.normal.clone()))
            .observe(update_material_on::<Pointer<Press>>(color.dark.clone()))
            .observe(update_material_on::<Pointer<Release>>(color.light.clone()))
            .insert(MeshMaterial2d(color.normal.clone()));

        self
    }
}

fn update_material_on<E: EntityEvent>(
    new_material: Handle<ColorMaterial>,
) -> impl Fn(On<E>, Query<&mut MeshMaterial2d<ColorMaterial>>) {
    move |event, mut query| {
        if let Ok(mut material) = query.get_mut(event.event_target()) {
            material.0 = new_material.clone();
        }
    }
}
