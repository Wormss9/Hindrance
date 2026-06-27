// use super::*;

// impl Board {
//     pub(crate) fn place_square_wall(
//         &mut self,
//         wall_id: usize,
//         wall_rotation: WallRotation,
//     ) -> Result<(), ()> {
//         let mut new_edges = self.edges.clone();
//         match wall_rotation {
//             WallRotation::Square(square_wall_rotation) => {
//                 let (xul, yul) = wall_get_xy(self.size, wall_id);

//                 match square_wall_rotation {
//                     SquareWallRotation::Horizontal => {
//                         remove_edge(
//                             &mut new_edges,
//                             tile_get_id(self.size, xul, yul),
//                             tile_get_id(self.size, xul, yul + 1),
//                         );
//                         remove_edge(
//                             &mut new_edges,
//                             tile_get_id(self.size, xul + 1, yul),
//                             tile_get_id(self.size, xul + 1, yul + 1),
//                         );
//                         //TODO VERIFY NOT CROSSING WALLS
//                         if xul > 0 {
//                             let conflict = wall_get_id(self.size, xul - 1, yul);
//                             if let Some(rot) = self.walls[conflict]
//                                 && rot == WallRotation::Square(SquareWallRotation::Horizontal)
//                             {
//                                 return Err(());
//                             }
//                         }
//                         if xul < self.size - 1 {
//                             let conflict = wall_get_id(self.size, xul + 1, yul);
//                             if let Some(rot) = self.walls[conflict]
//                                 && rot == WallRotation::Square(SquareWallRotation::Horizontal)
//                             {
//                                 return Err(());
//                             }
//                         }
//                     }
//                     SquareWallRotation::Vertical => {
//                         remove_edge(
//                             &mut new_edges,
//                             tile_get_id(self.size, xul, yul),
//                             tile_get_id(self.size, xul + 1, yul),
//                         );
//                         remove_edge(
//                             &mut new_edges,
//                             tile_get_id(self.size, xul, yul + 1),
//                             tile_get_id(self.size, xul + 1, yul + 1),
//                         );
//                         //TODO VERIFY NOT CROSSING WALLS
//                     }
//                 }
//             }
//             WallRotation::Hexagon(_) => unimplemented!(),
//         }
//         if !self.can_players_reach(&new_edges) {
//             return Err(());
//         }

//         self.edges = new_edges;
//         self.walls[wall_id] = Some(wall_rotation);
//         Ok(())
//     }
//     pub(crate) fn rotate_square_board(&mut self) {
//         self.player_locations = self
//             .player_locations
//             .iter()
//             .map(|x| self.size * self.size - 1 - x)
//             .rev()
//             .collect();
//         self.edges = self
//             .edges
//             .iter()
//             .rev()
//             .map(|g| g.iter().map(|x| self.size * self.size - 1 - x).collect())
//             .collect();
//     }
// }

// impl BoardBuilder {
//     pub(crate) fn build_square(self) -> Board {
//         let player_locations = vec![self.size * self.size - self.size / 2 - 1, self.size];
//         let mut goal_tiles = vec![Vec::new(); 2];
//         for x in 0..self.size {
//             goal_tiles[0].push(x);
//             goal_tiles[1].push(x + self.size * (self.size - 1));
//         }
//         let walls = vec![None; (self.size - 1) * (self.size - 1)];
//         let mut edges = vec![Vec::new(); self.size * self.size];
//         let mut grid = vec![Vec::new(); self.size];
//         let mut tile_rotation_map = vec![Vec::new()];
//         for y in 0..self.size {
//             for x in 0..self.size {
//                 let target = tile_get_id(self.size, x, y);
//                 grid[y].push(target);
//                 tile_rotation_map[0].push(self.size * self.size - target - 1);
//                 if y > 0 {
//                     edges[target].push(tile_get_id(self.size, x, y - 1));
//                 }
//                 if y < self.size - 1 {
//                     edges[target].push(tile_get_id(self.size, x, y + 1));
//                 }
//                 if x > 0 {
//                     edges[target].push(tile_get_id(self.size, x - 1, y));
//                 }
//                 if x < self.size - 1 {
//                     edges[target].push(tile_get_id(self.size, x + 1, y));
//                 }
//             }
//         }
//         let mut wall_rotation_map = vec![Vec::new()];
//         for y in 0..self.size - 1 {
//             for x in 0..self.size - 1 {
//                 wall_rotation_map[0]
//                     .push((self.size - 1) * (self.size - 1) - x - y * (self.size) - 1);
//             }
//         }

//         Board {
//             shape: self.shape,
//             size: self.size,
//             player_locations,
//             goal_tiles,
//             walls,
//             edges,
//             grid,
//             tile_rotation_map,
//             wall_rotation_map,
//         }
//     }
// }

// fn tile_get_id(size: usize, x: usize, y: usize) -> usize {
//     x + size * y
// }
// fn tile_get_xy(size: usize, tile_id: usize) -> (usize, usize) {
//     let x = tile_id % size;
//     let y = tile_id / size;
//     (x, y)
// }
// fn wall_get_id(size: usize, x: usize, y: usize) -> usize {
//     tile_get_id(size - 1, x, y)
// }
// fn wall_get_xy(size: usize, wall_id: usize) -> (usize, usize) {
//     tile_get_xy(size - 1, wall_id)
// }

// #[derive(Clone, Copy, PartialEq, Eq)]
// pub enum SquareWallRotation {
//     Horizontal,
//     Vertical,
// }
