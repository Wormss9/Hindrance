use bevy::prelude::*;

use crate::{
    colors::{PointerColorInteraction, Theme},
    exit_menu::ExitMenuState,
    game_logic::{
        BoardParameters, Edges, Shape, SquareWall,
        bundles::{TileBundle, WallBundle},
        components::{Foe, GridLocation, Id, Own, SquareGapId, SquareGapLocation, Wall},
        observers::{OwnMovement, PointerInteraction},
        systems::update_reachable_tiles,
    },
    main_menu::GameState,
    shapes::arrow_mesh,
};

const SHAPE: Shape = Shape::Square;

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
    commands.insert_resource(Edges::new(SHAPE));
    let square_entity = commands
        .spawn((Transform::default(), Visibility::Visible))
        .with_children(|parent| {
            let board: BoardParameters = SHAPE.into();
            //Exit arrow
            parent
                .spawn((
                    Mesh2d(meshes.add(arrow_mesh(32.0))),
                    Transform {
                        translation: Vec3::new(
                            -board.offset_size * (board.mid - 2) as f32,
                            board.offset_size * (board.size - board.mid - 1) as f32,
                            0.,
                        ),
                        rotation: Quat::from_rotation_z(std::f32::consts::PI),
                        ..default()
                    },
                    Pickable::default(),
                ))
                .with_color_set(&theme.exit)
                .observe(
                    |_: On<Pointer<Release>>, mut exit_state: ResMut<NextState<ExitMenuState>>| {
                        exit_state.set(ExitMenuState::Exiting);
                    },
                );

            let mut wall_entities = Vec::with_capacity((board.size - 1) * (board.size - 1) * 2);

            for y in 0..board.size {
                for x in 0..board.size {
                    parent
                        .spawn(TileBundle::new(&mut meshes, &theme, x, y, SHAPE))
                        .with_pointer_interaction()
                        .with_move_own();

                    // RD
                    if y < board.size - 1 && x < board.size - 1 {
                        wall_entities.push(
                            parent
                                .spawn(WallBundle::new(
                                    &mut meshes,
                                    &theme,
                                    x,
                                    y,
                                    SHAPE,
                                    Wall::Square(SquareWall::Down),
                                ))
                                .id(),
                        );
                        parent
                            .spawn((
                                Mesh2d(meshes.add(Rectangle::new(
                                    board.offset_size - board.tile_size,
                                    board.tile_size / 2.,
                                ))),
                                Transform::from_translation(Vec3::new(
                                    board.offset_size * (x as f32 - board.mid as f32)
                                        + board.offset_size / 2.,
                                    board.offset_size * (board.mid as f32 - y as f32)
                                        - board.tile_size / 4.,
                                    0.,
                                )),
                                Pickable::default(),
                                SquareGapId::new(
                                    SHAPE.get_id(x, y).expect("TODO failed"),
                                    SquareGapLocation::RD,
                                    wall_entities[2 * x + 2 * (board.size - 1) * y],
                                ),
                            ))
                            .observe(show_wall::<Pointer<Over>>())
                            .observe(hide_wall::<Pointer<Out>>());
                    };
                    // DR
                    if y < board.size - 1 && x < board.size - 1 {
                        wall_entities.push(
                            parent
                                .spawn(WallBundle::new(
                                    &mut meshes,
                                    &theme,
                                    x,
                                    y,
                                    SHAPE,
                                    Wall::Square(SquareWall::Right),
                                ))
                                .id(),
                        );
                        parent
                            .spawn((
                                Mesh2d(meshes.add(Rectangle::new(
                                    board.tile_size / 2.,
                                    board.offset_size - board.tile_size,
                                ))),
                                Transform::from_translation(Vec3::new(
                                    board.offset_size * (x as f32 - board.mid as f32)
                                        + board.tile_size / 4.,
                                    board.offset_size * (board.mid as f32 - y as f32)
                                        - board.offset_size / 2.,
                                    0.,
                                )),
                                Pickable::default(),
                                SquareGapId::new(
                                    SHAPE.get_id(x, y).expect("TODO failed"),
                                    SquareGapLocation::DR,
                                    wall_entities[2 * x + 2 * (board.size - 1) * y + 1],
                                ),
                            ))
                            .observe(show_wall::<Pointer<Over>>())
                            .observe(hide_wall::<Pointer<Out>>());
                    };
                    // RU
                    if y > 0 && x < board.size - 1 {
                        parent
                            .spawn((
                                Mesh2d(meshes.add(Rectangle::new(
                                    board.offset_size - board.tile_size,
                                    board.tile_size / 2.,
                                ))),
                                Transform::from_translation(Vec3::new(
                                    board.offset_size * (x as f32 - board.mid as f32)
                                        + board.offset_size / 2.,
                                    board.offset_size * (board.mid as f32 - y as f32)
                                        + board.tile_size / 4.,
                                    0.,
                                )),
                                Pickable::default(),
                                SquareGapId::new(
                                    SHAPE.get_id(x, y).expect("TODO failed"),
                                    SquareGapLocation::RU,
                                    wall_entities[2 * x + 2 * (board.size - 1) * (y - 1)],
                                ),
                            ))
                            .observe(show_wall::<Pointer<Over>>())
                            .observe(hide_wall::<Pointer<Out>>());
                    };
                    // DL
                    if y < board.size - 1 && x > 0 {
                        parent
                            .spawn((
                                Mesh2d(meshes.add(Rectangle::new(
                                    board.tile_size / 2.,
                                    board.offset_size - board.tile_size,
                                ))),
                                Transform::from_translation(Vec3::new(
                                    board.offset_size * (x as f32 - board.mid as f32)
                                        - board.tile_size / 4.,
                                    board.offset_size * (board.mid as f32 - y as f32)
                                        - board.offset_size / 2.,
                                    0.,
                                )),
                                Pickable::default(),
                                SquareGapId::new(
                                    SHAPE.get_id(x, y).expect("TODO failed"),
                                    SquareGapLocation::DL,
                                    wall_entities[2 * x + 2 * (board.size - 1) * y - 1],
                                ),
                            ))
                            .observe(show_wall::<Pointer<Over>>())
                            .observe(hide_wall::<Pointer<Out>>());
                    };
                }
            }
            parent.spawn((
                Mesh2d(meshes.add(Circle::new(board.tile_size / 2.))),
                transform_from_position(board.mid, 0, board.offset_size, board.mid),
                Pickable::default(),
                MeshMaterial2d(theme.foe.normal.clone()),
                GridLocation::new(board.mid, 0),
                Id(4),
                Foe,
            ));
            parent
                .spawn((
                    Mesh2d(meshes.add(Circle::new(board.tile_size / 2.))),
                    transform_from_position(
                        board.mid,
                        board.size - 1,
                        board.offset_size,
                        board.mid,
                    ),
                    Pickable::default(),
                    MeshMaterial2d(theme.own.normal.clone()),
                    GridLocation::new(board.mid, board.size - 1),
                    Id(76),
                    Own,
                ))
                .observe(show_wall::<Pointer<Over>>())
                .observe(hide_wall::<Pointer<Out>>());
        })
        .id();

    commands.insert_resource(SquareData { square_entity });
}

fn cleanup_square(mut commands: Commands, square_data: Res<SquareData>) {
    commands.entity(square_data.square_entity).despawn();
}

fn transform_from_position(x: usize, y: usize, offset_size: f32, mid: usize) -> Transform {
    Transform::from_translation(Vec3::new(
        offset_size * (x as f32 - mid as f32),
        offset_size * (mid as f32 - y as f32),
        0.,
    ))
}
