use bevy::prelude::*;

use crate::{
    colors::{PointerColorInteraction, Theme},
    game_logic::{
        FoeLocation, Interactable, OwnLocation, OwnMovement, Pointable, PointerInteraction,
        SquareGapId, SquareGapLocation, Tile,
    },
    grid::Edges,
    main_menu::GameState,
    shapes::arrow_mesh,
};

const SIZE: usize = 9;

pub struct SquarePlugin;

impl Plugin for SquarePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Square), setup_square)
            .add_systems(OnExit(GameState::Square), cleanup_square)
            .add_systems(
                Update,
                update_reachable_tiles.run_if(in_state(GameState::Square)),
            );
    }
}

fn show_wall<E: EntityEvent>() -> impl Fn(On<E>, Query<&SquareGapId>, Query<&mut Visibility>) {
    move |event, gap_query, mut visibility_query| {
        let hovered_entity = event.event_target();

        let Ok(gap) = gap_query.get(hovered_entity) else {
            return;
        };

        if let Ok(mut visibility) = visibility_query.get_mut(gap.wall) {
            *visibility = Visibility::Visible;
        }
    }
}
fn hide_wall<E: EntityEvent>() -> impl Fn(On<E>, Query<&SquareGapId>, Query<&mut Visibility>) {
    move |event, gap_query, mut visibility_query| {
        let hovered_entity = event.event_target();

        let Ok(gap) = gap_query.get(hovered_entity) else {
            return;
        };

        if let Ok(mut visibility) = visibility_query.get_mut(gap.wall) {
            *visibility = Visibility::Hidden;
        }
    }
}

#[derive(Resource)]
struct SquareData {
    square_entity: Entity,
}

pub fn setup_square(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, theme: Res<Theme>) {
    commands.insert_resource(Edges::square(SIZE));
    let square_entity = commands
        .spawn((Transform::default(), Visibility::Visible))
        .with_children(|parent| {
            let sqr_size = 60.;
            let gap_size = 15.;
            let sqr_offset = sqr_size + gap_size;
            let mid = (SIZE / 2) as f32;

            parent
                .spawn((
                    Mesh2d(meshes.add(arrow_mesh(32.0))),
                    Transform {
                        translation: Vec3::new(
                            sqr_offset * (-mid - 2.),
                            -sqr_offset * (mid - (SIZE - 1) as f32),
                            0.,
                        ),
                        rotation: Quat::from_rotation_z(std::f32::consts::PI),
                        ..default()
                    },
                    Pickable::default(),
                ))
                .with_color_set(&theme.exit);
            let mut wall_entities = Vec::with_capacity((SIZE - 1) * (SIZE - 1) * 2);
            for y in 0..SIZE {
                for x in 0..SIZE {
                    let id = y * SIZE + x;
                    parent
                        .spawn((
                            Mesh2d(meshes.add(Rectangle::new(sqr_size, sqr_size))),
                            MeshMaterial2d(theme.tile.normal.clone()),
                            transform_from_position(x, y, sqr_offset, mid),
                            Pickable::default(),
                            Pointable::default(),
                            Tile::new(id, x, y),
                            Interactable(false),
                        ))
                        .with_pointer_interaction()
                        .with_move_own();

                    // RD
                    if y < SIZE - 1 && x < SIZE - 1 {
                        wall_entities.push(
                            parent
                                .spawn((
                                    Mesh2d(
                                        meshes.add(Rectangle::new(
                                            gap_size,
                                            sqr_size * 2. + gap_size,
                                        )),
                                    ),
                                    MeshMaterial2d(theme.wall.normal.clone()),
                                    Visibility::Hidden,
                                    Pickable::IGNORE,
                                    Transform::from_translation(Vec3::new(
                                        sqr_offset * (x as f32 - mid) + sqr_offset / 2.,
                                        sqr_offset * (mid - y as f32) - sqr_offset / 2.,
                                        0.,
                                    )),
                                ))
                                .id(),
                        );
                        parent
                            .spawn((
                                Mesh2d(
                                    meshes
                                        .add(Rectangle::new(sqr_offset - sqr_size, sqr_size / 2.)),
                                ),
                                Transform::from_translation(Vec3::new(
                                    sqr_offset * (x as f32 - mid) + sqr_offset / 2.,
                                    sqr_offset * (mid - y as f32) - sqr_size / 4.,
                                    0.,
                                )),
                                Pickable::default(),
                                SquareGapId::new(
                                    id,
                                    SquareGapLocation::RD,
                                    wall_entities[2 * x + 2 * (SIZE - 1) * y],
                                ),
                            ))
                            .observe(show_wall::<Pointer<Over>>())
                            .observe(hide_wall::<Pointer<Out>>());
                    };
                    // DR
                    if y < SIZE - 1 && x < SIZE - 1 {
                        wall_entities.push(
                            parent
                                .spawn((
                                    Mesh2d(
                                        meshes.add(Rectangle::new(
                                            sqr_size * 2. + gap_size,
                                            gap_size,
                                        )),
                                    ),
                                    MeshMaterial2d(theme.wall.normal.clone()),
                                    Visibility::Hidden,
                                    Pickable::IGNORE,
                                    Transform::from_translation(Vec3::new(
                                        sqr_offset * (x as f32 - mid) + sqr_offset / 2.,
                                        sqr_offset * (mid - y as f32) - sqr_offset / 2.,
                                        0.,
                                    )),
                                ))
                                .id(),
                        );
                        parent
                            .spawn((
                                Mesh2d(
                                    meshes
                                        .add(Rectangle::new(sqr_size / 2., sqr_offset - sqr_size)),
                                ),
                                Transform::from_translation(Vec3::new(
                                    sqr_offset * (x as f32 - mid) + sqr_size / 4.,
                                    sqr_offset * (mid - y as f32) - sqr_offset / 2.,
                                    0.,
                                )),
                                Pickable::default(),
                                SquareGapId::new(
                                    id,
                                    SquareGapLocation::DR,
                                    wall_entities[2 * x + 2 * (SIZE - 1) * y + 1],
                                ),
                            ))
                            .observe(show_wall::<Pointer<Over>>())
                            .observe(hide_wall::<Pointer<Out>>());
                    };
                    // RU
                    if y > 0 && x < SIZE - 1 {
                        parent
                            .spawn((
                                Mesh2d(
                                    meshes
                                        .add(Rectangle::new(sqr_offset - sqr_size, sqr_size / 2.)),
                                ),
                                Transform::from_translation(Vec3::new(
                                    sqr_offset * (x as f32 - mid) + sqr_offset / 2.,
                                    sqr_offset * (mid - y as f32) + sqr_size / 4.,
                                    0.,
                                )),
                                Pickable::default(),
                                SquareGapId::new(
                                    id,
                                    SquareGapLocation::RU,
                                    wall_entities[2 * x + 2 * (SIZE - 1) * (y - 1)],
                                ),
                            ))
                            .observe(show_wall::<Pointer<Over>>())
                            .observe(hide_wall::<Pointer<Out>>());
                    };
                    // DL
                    if y < SIZE - 1 && x > 0 {
                        parent
                            .spawn((
                                Mesh2d(
                                    meshes
                                        .add(Rectangle::new(sqr_size / 2., sqr_offset - sqr_size)),
                                ),
                                Transform::from_translation(Vec3::new(
                                    sqr_offset * (x as f32 - mid) - sqr_size / 4.,
                                    sqr_offset * (mid - y as f32) - sqr_offset / 2.,
                                    0.,
                                )),
                                Pickable::default(),
                                SquareGapId::new(
                                    id,
                                    SquareGapLocation::DL,
                                    wall_entities[2 * x + 2 * (SIZE - 1) * y - 1],
                                ),
                            ))
                            .observe(show_wall::<Pointer<Over>>())
                            .observe(hide_wall::<Pointer<Out>>());
                    };
                }
            }
            parent.spawn((
                Mesh2d(meshes.add(Circle::new(sqr_size / 2.))),
                transform_from_position(mid as usize, 0, sqr_offset, mid),
                Pickable::default(),
                MeshMaterial2d(theme.foe.normal.clone()),
                FoeLocation::new(mid as usize, 0),
            ));
            parent
                .spawn((
                    Mesh2d(meshes.add(Circle::new(sqr_size / 2.))),
                    transform_from_position(mid as usize, SIZE - 1, sqr_offset, mid),
                    Pickable::default(),
                    MeshMaterial2d(theme.own.normal.clone()),
                    OwnLocation::new(mid as usize, SIZE - 1),
                ))
                .observe(show_wall::<Pointer<Over>>())
                .observe(hide_wall::<Pointer<Out>>());
        })
        .id();

    commands.insert_resource(SquareData { square_entity });
}

fn update_reachable_tiles(
    mut query: Query<(
        &mut MeshMaterial2d<ColorMaterial>,
        &Pointable,
        &Tile,
        &mut Interactable,
    )>,
    own_query: Query<&OwnLocation>,
    foe_query: Query<&FoeLocation>,
    theme: Res<Theme>,
    edges: Res<Edges>,
) {
    let Ok(own_location) = own_query.single() else {
        return;
    };

    let Ok(foe_location) = foe_query.single() else {
        return;
    };

    let reachable_ids = edges.reachable_from(own_location, foe_location);

    for (mut material, pointable, tile, mut interactable) in &mut query {
        let reachable = reachable_ids.contains(&tile.id);

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

fn clean_reachable_tiles(
    mut query: Query<
        (&mut MeshMaterial2d<ColorMaterial>, &mut Interactable),
        (With<Pointable>, With<Tile>),
    >,
    theme: Res<Theme>,
) {
    for (mut material, mut interactable) in &mut query {
        material.0 = theme.tile.normal.clone();
        interactable.0 = false;
    }
}

fn cleanup_square(mut commands: Commands, square_data: Res<SquareData>) {
    commands.entity(square_data.square_entity).despawn();
}

fn transform_from_position(x: usize, y: usize, sqr_offset: f32, mid: f32) -> Transform {
    Transform::from_translation(Vec3::new(
        sqr_offset * (x as f32 - mid),
        sqr_offset * (mid - y as f32),
        0.,
    ))
}
