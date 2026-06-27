// mod hexagon;
// mod square;

// use std::{collections::VecDeque, iter};

// use enum_dispatch::enum_dispatch;
// use hexagon::*;

// use crate::square::SquareWallRotation;

// pub struct BoardBuilder {
//     shape: Shape,
//     size: usize,
// }

// impl BoardBuilder {
//     pub fn build(self) -> Board {
//         match &self.shape {
//             Shape::Square => self.build_hexagon(),
//             Shape::Hexagon => self.build_square(),
//         }
//     }
// }

use bitcode::{Decode, Encode};

#[derive(Encode, Decode, Clone, Copy)]
pub enum Shape {
    Square,
    Hexagon,
}

impl Shape {
    pub fn max_players(self) -> u8 {
        match self {
            Shape::Square => 2,
            Shape::Hexagon => 3,
        }
    }
}

// pub struct Board {
//     pub shape: Shape,
//     pub size: usize,
//     pub player_locations: Vec<usize>,
//     pub goal_tiles: Vec<Vec<usize>>,
//     pub walls: Vec<Option<WallRotation>>,
//     pub edges: Vec<Vec<usize>>,
//     pub grid: Vec<Vec<usize>>,
//     tile_rotation_map: Vec<Vec<usize>>,
//     wall_rotation_map: Vec<Vec<usize>>,
// }

// #[derive(Clone, Copy, PartialEq, Eq)]
// pub enum WallRotation {
//     Square(SquareWallRotation),
//     Hexagon(HexagonWallRotation),
// }

// impl From<BoardBuilder> for Board {
//     fn from(value: BoardBuilder) -> Self {
//         match value.shape {
//             Shape::Square => value.build_square(),
//             Shape::Hexagon => value.build_hexagon(),
//         }
//     }
// }

// impl Board {
//     pub fn rotate_tile(&self, tile_id: usize, player: usize) -> usize {
//         if player == 0 {
//             return tile_id;
//         }
//         self.tile_rotation_map[player][tile_id]
//     }
//     pub fn rotate_wall(
//         &self,
//         wall_id: usize,
//         player: usize,
//         wall_rotation: WallRotation,
//     ) -> (usize, WallRotation) {
//         if player == 0 {
//             return (wall_id, wall_rotation);
//         }
//         let rotation = match wall_rotation {
//             WallRotation::Square(square_wall_rotation) => {
//                 WallRotation::Square(square_wall_rotation)
//             }
//             WallRotation::Hexagon(hexagon_wall_rotation) => {
//                 WallRotation::Hexagon(hexagon_wall_rotation + player)
//             }
//         };

//         (self.wall_rotation_map[player][wall_id], rotation)
//     }
//     pub fn place_wall(&mut self, wall_id: usize, wall_rotation: WallRotation) -> Result<(), ()> {
//         match &self.shape {
//             Shape::Square => self.place_square_wall(wall_id, wall_rotation),
//             Shape::Hexagon => self.place_hex_wall(wall_id, wall_rotation),
//         }
//     }
//     fn can_players_reach(&self, edges: &[Vec<usize>]) -> bool {
//         for (&player, goals) in self.player_locations.iter().zip(&self.goal_tiles) {
//             if !can_reach(edges, player, goals) {
//                 return false;
//             }
//         }
//         true
//     }
// }

// fn can_reach(edges: &[Vec<usize>], player: usize, goals: &[usize]) -> bool {
//     let mut visited = vec![false; edges.len()];
//     let mut queue = VecDeque::new();

//     visited[player] = true;
//     queue.push_back(player);

//     while let Some(tile) = queue.pop_front() {
//         if goals.contains(&tile) {
//             return true;
//         }

//         for &next in edges[tile].iter() {
//             if !visited[next] {
//                 visited[next] = true;
//                 queue.push_back(next);
//             }
//         }
//     }

//     false
// }

// pub(crate) fn remove_edge(edges: &mut [Vec<usize>], a: usize, b: usize) {
//     edges[a].retain(|&x| x != b);
//     edges[b].retain(|&x| x != a);
// }
