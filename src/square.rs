use bevy::prelude::*;

use crate::{
    colors::{PointerInteraction, Theme},
    game_logic::{PlayerLocation, SquareGapId, SquareGapLocation, TileId},
    main_menu::GameState,
};

const SIZE: usize = 9;

pub struct SquarePlugin;

impl Plugin for SquarePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Square), setup_square)
            .add_systems(OnExit(GameState::Square), cleanup_square);
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
    let square_entity = commands
        .spawn((Transform::default(), Visibility::Visible))
        .with_children(|parent| {
            let sqr_size = 60.;
            let gap_size = 15.;
            let sqr_offset = sqr_size + gap_size;
            let mid = (SIZE / 2) as f32;
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
                            TileId(id),
                        ))
                        .with_color_set(&theme.tile);
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
            parent
                .spawn((
                    Mesh2d(meshes.add(Circle::new(sqr_size / 2.))),
                    transform_from_position(mid as usize, 0, sqr_offset, mid),
                    Pickable::default(),
                    MeshMaterial2d(theme.foe.normal.clone()),
                    PlayerLocation::new(mid as usize, 0),
                ))
                .with_color_set(&theme.foe);
            println!("{} {}", mid as usize, SIZE);
            parent
                .spawn((
                    Mesh2d(meshes.add(Circle::new(sqr_size / 2.))),
                    transform_from_position(mid as usize, SIZE - 1, sqr_offset, mid),
                    Pickable::default(),
                    MeshMaterial2d(theme.own.normal.clone()),
                    PlayerLocation::new(mid as usize, SIZE),
                ))
                .observe(show_wall::<Pointer<Over>>())
                .observe(hide_wall::<Pointer<Out>>())
                .with_color_set(&theme.own);
        })
        .id();

    commands.insert_resource(SquareData { square_entity });
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
