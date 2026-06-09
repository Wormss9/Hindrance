use crate::game_logic::{
    Board, Edges, SquareWall, TriangleWall, WallCount,
    components::{Gap, GridLocation, Id, Interactable, Own, Pointable, Tile, Wall},
};
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
    target_query: Query<(&Id, &GridLocation, &Wall), With<Gap>>,
    wall_query: Query<(Entity, &Id, &Wall), With<Wall>>,
    gap_query: Query<(Entity, &Gap)>,
    wall_count: ResMut<WallCount>,
    edges: ResMut<Edges>,
    board: Res<Board>,
) {
    let target = event.event_target();
    let (id, location, rotation) = target_query
        .get(target)
        .expect("Failed to get clicked gap!");
    add_wall(
        commands, *id, *location, *rotation, wall_query, gap_query, wall_count, edges, board,
    );
}
#[allow(clippy::too_many_arguments)]
fn add_wall(
    mut commands: Commands,
    id: Id,
    location: GridLocation,
    rotation: Wall,
    wall_query: Query<(Entity, &Id, &Wall), With<Wall>>,
    gap_query: Query<(Entity, &Gap)>,
    mut wall_count: ResMut<WallCount>,
    mut edges: ResMut<Edges>,
    board: Res<Board>,
) {
    if wall_count.own == 0 {
        return;
    }
    let mut walls = Vec::new();

    match board.shape {
        super::Shape::Square => {
            for location in SquareWall::iter() {
                walls.push((id, Wall::Square(location)))
            }
        }
        super::Shape::Triangle => {
            for location in TriangleWall::iter() {
                walls.push((id, Wall::Triangle(location)))
            }
        }
    }
    let GridLocation { x, y } = location;
    match rotation {
        Wall::Square(square_wall) => match square_wall {
            SquareWall::Right => {
                if y > 0
                    && let Some(id) = board.get_tile_id(x, y - 1)
                {
                    walls.push((Id(id), Wall::Square(square_wall)))
                }
                if let Some(id) = board.get_tile_id(x, y + 1) {
                    walls.push((Id(id), Wall::Square(square_wall)))
                }
            }
            SquareWall::Down => {
                if x > 0
                    && let Some(id) = board.get_tile_id(x - 1, y)
                {
                    walls.push((Id(id), Wall::Square(square_wall)))
                }
                if let Some(id) = board.get_tile_id(x + 1, y) {
                    walls.push((Id(id), Wall::Square(square_wall)))
                }
            }
        },
        Wall::Triangle(triangle_wall) => match triangle_wall {
            TriangleWall::Down => {
                if x > 1
                    && let Some(id) = board.get_tile_id(x - 2, y)
                {
                    walls.push((Id(id), Wall::Triangle(triangle_wall)))
                }
                if let Some(id) = board.get_tile_id(x + 2, y) {
                    walls.push((Id(id), Wall::Triangle(triangle_wall)))
                }
            }
            TriangleWall::UpRight => {
                if y > 0
                    && x > 0
                    && let Some(id) = board.get_tile_id(x, y - 1)
                {
                    walls.push((Id(id), Wall::Triangle(triangle_wall)))
                }
                if let Some(id) = board.get_tile_id(x, y + 1) {
                    walls.push((Id(id), Wall::Triangle(triangle_wall)))
                }
            }
            TriangleWall::DownRight => {
                if y > 0
                    && let Some(id) = board.get_tile_id(x + 2, y - 1)
                {
                    walls.push((Id(id), Wall::Triangle(triangle_wall)))
                }
                if x > 1
                    && let Some(id) = board.get_tile_id(x - 2, y + 1)
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

    // let tile_base = wall / 2;
    // let y = tile_base / (size - 1);
    // let x = tile_base % (size - 1);

    // let tile_id = x + y * size;
    // let tile_bellow_id = tile_id + size;

    // if wall.is_multiple_of(2) {
    //     edges.remove_edge(tile_id, tile_id + 1);
    //     edges.remove_edge(tile_bellow_id, tile_bellow_id + 1);
    // } else {
    //     edges.remove_edge(tile_id, tile_bellow_id);
    //     edges.remove_edge(tile_id + 1, tile_bellow_id + 1);
    // };
    wall_count.own -= 1;
}
