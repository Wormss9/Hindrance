use bevy::prelude::*;

use crate::{
    colors::{PointerColorInteraction, Theme},
    exit_menu::ExitMenuState,
    game_logic::{
        Owner, bundles::*, components::*, enums::*, observers::*, resources::*, systems::*,
    },
    main_menu::GameState,
    shapes::arrow_mesh,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Owner>()
            .add_systems(OnEnter(GameState::InGame), setup_game)
            .add_systems(OnExit(GameState::InGame), cleanup_square)
            .add_systems(
                Update,
                update_reachable_tiles.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                update_counter_text
                    .after(setup_game)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Resource)]
struct GameData {
    square_entity: Entity,
}

pub fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Assets<ColorMaterial>>,
    theme: Res<Theme>,
    board: Res<Board>,
) {
    let board = *board;
    commands.insert_resource(Into::<Edges>::into(board));
    commands.insert_resource(WallCount::new(10));
    let square_entity = commands
        .spawn((Transform::default(), Visibility::Visible))
        .with_children(|parent| {
            // Exit arrow
            let (x_mid, y_mid) = board.get_mids();
            parent
                .spawn((
                    Mesh2d(meshes.add(arrow_mesh(board.tile_size))),
                    Transform {
                        translation: Vec3::new(
                            -board.get_x_offset() * (x_mid + 2) as f32,
                            board.get_y_offset() * y_mid as f32,
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

            // Counters
            let own_color = materials.get(&theme.own.normal).unwrap().color;
            let foe_colors = match board.shape {
                Shape::Square => {
                    vec![materials.get(&theme.foe1.normal).unwrap().color]
                }
                Shape::Triangle => vec![
                    materials.get(&theme.foe1.normal).unwrap().color,
                    materials.get(&theme.foe2.normal).unwrap().color,
                ],
            };
            parent.spawn((
                Text2d::new(board.max_walls.to_string()),
                TextColor(own_color),
                TextFont {
                    font_size: board.tile_size,
                    ..default()
                },
                Transform {
                    translation: Vec3::new(
                        -board.get_x_offset() * (board.get_mids().0 + 2) as f32,
                        0.,
                        0.,
                    ),
                    ..default()
                },
                CounterText::OWN,
            ));
            let foe_ammount = foe_colors.len() as f32;
            for (foe_number, foe_color) in foe_colors.into_iter().enumerate() {
                parent.spawn((
                    Text2d::new(board.max_walls.to_string()),
                    TextColor(foe_color),
                    TextFont {
                        font_size: board.tile_size,
                        ..default()
                    },
                    Transform {
                        translation: Vec3::new(
                            board.get_x_offset() * (board.get_mids().0 + 2) as f32,
                            -board.get_y_offset() * ((foe_ammount - 1.) / 2. - foe_number as f32),
                            0.,
                        ),
                        ..default()
                    },
                    CounterText::FOE,
                    Id(foe_number),
                ));
            }

            // Tiles Walls Gaps
            let (x_size, y_size) = board.grid_dimentions();

            for y in 0..y_size {
                for x in 0..x_size {
                    if board.get_tile_id(x, y).is_none() {
                        continue;
                    }
                    // Tiles
                    let goal = board.goal(x, y);
                    parent
                        .spawn(TileBundle::new(&mut meshes, &theme, x, y, board, goal))
                        .with_pointer_interaction()
                        .observe(move_own);

                    // Walls
                    let mut wall_entities: Vec<(Entity, Wall)> =
                        Vec::with_capacity(match board.shape {
                            Shape::Square => 2,
                            Shape::Triangle => 3,
                        });

                    for wall_position in board.get_walls(x, y) {
                        let wall_entity = parent
                            .spawn(WallBundle::new(
                                &mut meshes,
                                &theme,
                                x,
                                y,
                                board,
                                wall_position,
                            ))
                            .id();
                        wall_entities.push((wall_entity, wall_position));
                    }

                    // Gaps
                    for (wall_entity, wall) in wall_entities {
                        parent
                            .spawn(GapBundle::new(
                                &mut meshes,
                                board,
                                x,
                                y,
                                wall_entity,
                                wall,
                                true,
                            ))
                            .observe(show_wall)
                            .observe(hide_wall)
                            .observe(place_wall);
                        parent
                            .spawn(GapBundle::new(
                                &mut meshes,
                                board,
                                x,
                                y,
                                wall_entity,
                                wall,
                                false,
                            ))
                            .observe(show_wall)
                            .observe(hide_wall)
                            .observe(place_wall);
                    }
                }
            }
            match board.shape {
                Shape::Square => {
                    let (x, y) = (board.size / 2, 0);
                    parent.spawn((
                        Mesh2d(meshes.add(Circle::new(board.tile_size / 2.))),
                        board.into_tile_transform(x, y),
                        Pickable::default(),
                        MeshMaterial2d(theme.foe1.normal.clone()),
                        GridLocation::new(x, y),
                        Id(board.get_tile_id(x, y).expect("Failed to spawn foe1")),
                        Owner::Foe1,
                        Character,
                    ));
                    let (x, y) = (board.size / 2, board.size - 1);
                    parent.spawn((
                        Mesh2d(meshes.add(Circle::new(board.tile_size / 2.))),
                        board.into_tile_transform(x, y),
                        Pickable::default(),
                        MeshMaterial2d(theme.own.normal.clone()),
                        GridLocation::new(x, y),
                        Id(board.get_tile_id(x, y).expect("Failed to spawn own")),
                        Owner::Own,
                        Character,
                    ));
                }
                Shape::Triangle => {
                    let (x, y) = (board.size, board.size / 2);
                    parent.spawn((
                        Mesh2d(meshes.add(Circle::new(board.tile_size / 3.))),
                        board.into_tile_transform(x, y),
                        Pickable::default(),
                        MeshMaterial2d(theme.foe1.normal.clone()),
                        GridLocation::new(x, y),
                        Id(board.get_tile_id(x, y).expect("Failed to spawn foe1")),
                        Owner::Foe1,
                        Character,
                    ));
                    let gap = match board.size.is_multiple_of(2) {
                        true => 2,
                        false => 1,
                    };
                    let (x, y) = (4 * board.size - gap, board.size / 2);
                    parent.spawn((
                        Mesh2d(meshes.add(Circle::new(board.tile_size / 3.))),
                        board.into_tile_transform(x, y),
                        Pickable::default(),
                        MeshMaterial2d(theme.foe2.normal.clone()),
                        GridLocation::new(x, y),
                        Id(board.get_tile_id(x, y).expect("Failed to spawn foe2")),
                        Owner::Foe2,
                        Character,
                    ));
                    let (x, y) = (board.size, 2 * board.size - 1);
                    parent.spawn((
                        Mesh2d(meshes.add(Circle::new(board.tile_size / 3.))),
                        board.into_tile_transform(x, y),
                        Pickable::default(),
                        MeshMaterial2d(theme.own.normal.clone()),
                        GridLocation::new(x, y),
                        Id(board.get_tile_id(x, y).expect("Failed to spawn own")),
                        Owner::Own,
                        Character,
                    ));
                }
            }
        })
        .id();
    commands.insert_resource(GameData { square_entity });
}

fn cleanup_square(mut commands: Commands, square_data: Res<GameData>) {
    commands.remove_resource::<Edges>();
    commands.remove_resource::<WallCount>();
    commands.entity(square_data.square_entity).despawn();
}
