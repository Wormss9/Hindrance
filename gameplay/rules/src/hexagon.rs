// use super::*;

// impl Board {
//     pub(crate) fn place_hex_wall(
//         &mut self,
//         wall_id: usize,
//         wall_rotation: WallRotation,
//     ) -> Result<(), ()> {
//         todo!()
//     }
// }

// impl BoardBuilder {
//     pub(crate) fn build_hexagon(self) -> Board {
//         // let owned_tiles = vec![Vec::new(); 3];
//         // let walls = vec![Vec::new(); 2 * self.size - 1];
//         // let edges = vec![Vec::new();self.size*2];

//         // let players = vec![];

//         todo!()

//         // let mut hexagon: Vec<Vec<usize>> = vec![vec![]; 2 * size];
//         // let (x_size, _) = (size * 4, size * 2);
//         // for (y, row) in hexagon.iter_mut().enumerate() {
//         //     for x in 0..x_size {
//         //         match get_id(size, x, y) {
//         //             Some(id) => row.push(id),
//         //             None => continue,
//         //         }
//         //     }
//         // }

//         // let rotation1 = Hexagon::rotate_hexagon(&hexagon, size);
//         // let rotation2 = Hexagon::rotate_hexagon(&rotation1, size);

//         // let rotation_map = HashMap::from([
//         //     (Owner::Own, hexagon.clone()),
//         //     (Owner::Foe1, rotation1),
//         //     (Owner::Foe2, rotation2),
//         // ]);

//         // Board {
//         //     shape: self.shape,
//         //     size: self.size,
//         //     players,
//         //     owned_tiles,
//         //     walls,
//         //     edges,
//         //     grid,
//         //     tile_rotation_map,
//         //     wall_rotation_map,
//         // }
//     }
// }

// #[derive(Clone, Copy, PartialEq, Eq)]
// pub enum HexagonWallRotation {
//     Horizontal = 0,
//     Left = 1,
//     Right = 2,
// }

// impl std::ops::Add<usize> for HexagonWallRotation {
//     type Output = HexagonWallRotation;

//     fn add(self, rhs: usize) -> Self::Output {
//         match (self as usize + rhs) % 3 {
//             0 => HexagonWallRotation::Horizontal,
//             1 => HexagonWallRotation::Left,
//             2 => HexagonWallRotation::Right,
//             _ => unreachable!(),
//         }
//     }
// }

// fn front_skip(x: usize, y: usize, size: usize) -> bool {
//     size > y && x < 2 * (size - y) - 1
// }

// fn back_skip(x: usize, y: usize, size: usize) -> bool {
//     y >= size && x > 6 * size - 2 * y - 2
// }
