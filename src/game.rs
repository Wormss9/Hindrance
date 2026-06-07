use bevy::prelude::*;

use crate::{
    colors::{PointerColorInteraction, Theme},
    exit_menu::ExitMenuState,
    game_logic::{
        Board, Edges, SquareWall, WallCount,
        bundles::{SquareGapBundle, TileBundle, WallBundle},
        components::{CounterText, Foe, GridLocation, Id, Own, SquareGapPosition, Wall},
        observers::{PointerInteraction, hide_wall, move_own, place_wall, show_wall},
        systems::{update_counter_text, update_reachable_tiles},
    },
    main_menu::GameState,
    shapes::arrow_mesh,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_game)
            .add_systems(OnExit(GameState::InGame), cleanup_square);
        // .add_systems(
        //     Update,
        //     update_reachable_tiles.run_if(in_state(GameState::InGame)),
        // )
        // .add_systems(
        //     Update,
        //     update_counter_text
        //         .after(setup_square)
        //         .run_if(in_state(GameState::InGame)),
        // );
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
            //Exit arrow
            // parent
            //     .spawn((
            //         Mesh2d(meshes.add(arrow_mesh(board.tile_size))),
            //         Transform {
            //             translation: Vec3::new(
            //                 -shape.get_x_offset() * (board.mid + 2) as f32,
            //                 shape.get_y_offset() * (board.size - board.mid - 1) as f32,
            //                 0.,
            //             ),
            //             rotation: Quat::from_rotation_z(std::f32::consts::PI),
            //             ..default()
            //         },
            //         Pickable::default(),
            //     ))
            //     .with_color_set(&theme.exit)
            //     .observe(
            //         |_: On<Pointer<Release>>, mut exit_state: ResMut<NextState<ExitMenuState>>| {
            //             exit_state.set(ExitMenuState::Exiting);
            //         },
            //     );

            //Counters
            // let own_color = materials.get(&theme.own.normal).unwrap().color;
            // let foe1_color = materials.get(&theme.foe1.normal).unwrap().color;
            // parent.spawn((
            //     Text2d::new("10"),
            //     TextColor(own_color),
            //     TextFont {
            //         font_size: board.tile_size,
            //         ..default()
            //     },
            //     Transform {
            //         translation: Vec3::new(-shape.get_x_offset() * (board.mid + 2) as f32, 0., 0.),
            //         ..default()
            //     },
            //     CounterText::OWN,
            // ));
            // parent.spawn((
            //     Text2d::new("10"),
            //     TextColor(foe1_color),
            //     TextFont {
            //         font_size: board.tile_size,
            //         ..default()
            //     },
            //     Transform {
            //         translation: Vec3::new(shape.get_x_offset() * (board.mid + 2) as f32, 0., 0.),
            //         ..default()
            //     },
            //     CounterText::FOE,
            // ));

            let mut wall_entities = Vec::with_capacity((board.size - 1) * (board.size - 1) * 2);

            let (x_size, y_size) = board.grid_dimentions();

            for y in 0..y_size {
                for x in 0..x_size {
                    if board.get_tile_id(x, y).is_none() {
                        continue;
                    }
                    parent
                        .spawn(TileBundle::new(&mut meshes, &theme, x, y, board))
                        .with_pointer_interaction()
                        .observe(move_own);

                    if y < board.size - 1 && x < board.size - 1 {
                        wall_entities.push(
                            parent
                                .spawn(WallBundle::new(
                                    &mut meshes,
                                    &theme,
                                    x,
                                    y,
                                    board,
                                    Wall::Square(SquareWall::Right),
                                ))
                                .id(),
                        );
                    }
                }
            }
            //             parent
            //                 .spawn(SquareGapBundle::new(
            //                     &mut meshes,
            //                     shape,
            //                     x,
            //                     y,
            //                     SquareGapPosition::RD,
            //                     &wall_entities,
            //                 ))
            //                 .observe(show_wall)
            //                 .observe(hide_wall)
            //                 .observe(place_wall(shape));
            //         };
            //         if y < board.size - 1 && x < board.size - 1 {
            //             wall_entities.push(
            //                 parent
            //                     .spawn(WallBundle::new(
            //                         &mut meshes,
            //                         &theme,
            //                         x,
            //                         y,
            //                         shape,
            //                         Wall::Square(SquareWall::Down),
            //                     ))
            //                     .id(),
            //             );
            //             parent
            //                 .spawn(SquareGapBundle::new(
            //                     &mut meshes,
            //                     shape,
            //                     x,
            //                     y,
            //                     SquareGapPosition::DR,
            //                     &wall_entities,
            //                 ))
            //                 .observe(show_wall)
            //                 .observe(hide_wall)
            //                 .observe(place_wall(shape));
            //         };
            //         if y > 0 && x < board.size - 1 {
            //             parent
            //                 .spawn(SquareGapBundle::new(
            //                     &mut meshes,
            //                     shape,
            //                     x,
            //                     y,
            //                     SquareGapPosition::RU,
            //                     &wall_entities,
            //                 ))
            //                 .observe(show_wall)
            //                 .observe(hide_wall)
            //                 .observe(place_wall(shape));
            //         };
            //         if y < board.size - 1 && x > 0 {
            //             parent
            //                 .spawn(SquareGapBundle::new(
            //                     &mut meshes,
            //                     shape,
            //                     x,
            //                     y,
            //                     SquareGapPosition::DL,
            //                     &wall_entities,
            //                 ))
            //                 .observe(show_wall)
            //                 .observe(hide_wall)
            //                 .observe(place_wall(shape));
            //         };
            //     }
            // }
            // parent.spawn((
            //     Mesh2d(meshes.add(Circle::new(board.tile_size / 2.))),
            //     shape.into_tile_transform(board.mid, 0),
            //     Pickable::default(),
            //     MeshMaterial2d(theme.foe1.normal.clone()),
            //     GridLocation::new(board.mid, 0),
            //     Id(4),
            //     Foe,
            // ));
            // parent
            //     .spawn((
            //         Mesh2d(meshes.add(Circle::new(board.tile_size / 2.))),
            //         shape.into_tile_transform(board.mid, board.size - 1),
            //         Pickable::default(),
            //         MeshMaterial2d(theme.own.normal.clone()),
            //         GridLocation::new(board.mid, board.size - 1),
            //         Id(76),
            //         Own,
            //     ))
            //     .observe(show_wall)
            //     .observe(hide_wall);
        })
        .id();

    commands.insert_resource(GameData { square_entity });
}

fn cleanup_square(mut commands: Commands, square_data: Res<GameData>) {
    commands.remove_resource::<Edges>();
    commands.remove_resource::<WallCount>();
    commands.entity(square_data.square_entity).despawn();
}
