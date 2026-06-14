use bevy::{math::usize, platform::collections::HashMap};

use crate::game::Owner;

use super::ShapeTrait;

#[derive(Clone)]
pub struct Hexagon {
    pub hexagon: Vec<Vec<usize>>,
    pub size: usize,
    rotation_map: HashMap<Owner, Vec<Vec<usize>>>,
}

impl Hexagon {
    pub fn is_triangle_rotated(&self, x: usize, y: usize) -> Option<bool> {
        self.get_id(x, y)
            .map(|id| (id + y).is_multiple_of(2) ^ (y >= self.size()))
    }
    pub fn get_triangle_rotation_offset(&self, tile_size: f32) -> f32 {
        (3.0_f32).sqrt() * tile_size / 6.
    }
    fn rotate_hexagon(hexagon: &Vec<Vec<usize>>, size: usize) -> Vec<Vec<usize>> {
        let mut hexagon = hexagon.clone();
        let mut rotated = vec![Vec::new(); size * 2];
        let y_max = size * 2;
        let mid = (y_max as f32 - 1.0) / 2.0;
        for y in 0..size * 2 {
            let x_pairs = (y_max as f32 - (y as f32 - mid).abs()) as usize + 1;
            let empty_x = hexagon.iter().filter(|inner| inner.is_empty()).count();
            for x_off in 0..x_pairs {
                let x = x_off + empty_x;
                if y < y_max / 2 && x == x_pairs - 1 || y >= y_max / 2 && x_off == 0 {
                    let a = hexagon[x].pop().unwrap();
                    rotated[y].push(a);
                } else {
                    let b = hexagon[x].pop().unwrap();
                    let a = hexagon[x].pop().unwrap();
                    rotated[y].push(a);
                    rotated[y].push(b);
                }
            }
        }

        rotated.iter_mut().for_each(|row| row.reverse());
        rotated.reverse();

        rotated
    }
}

impl ShapeTrait for Hexagon {
    fn get_id(&self, x: usize, y: usize) -> Option<usize> {
        get_id(self.size, x, y)
    }

    fn get_local_xy(&self, id: usize) -> Option<(usize, usize)> {
        for (y, row) in self.hexagon.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if id == *tile {
                    return Some((x, y));
                }
            }
        }
        None
    }

    fn size(&self) -> usize {
        self.size
    }

    fn goal(&self, x: usize, y: usize) -> Owner {
        if y == 0 {
            Owner::Own
        } else if y > self.grid_dimensions().1 / 2 - 2 {
            if x == 0 || x == 1 {
                Owner::Foe2
            } else if x == self.size() * 2 + (self.grid_dimensions().1 - y) * 2 - 3
                || x == self.size() * 2 + (self.grid_dimensions().1 - y) * 2 - 2
            {
                Owner::Foe1
            } else {
                Owner::None
            }
        } else {
            Owner::None
        }
    }

    fn grid_dimensions(&self) -> (usize, usize) {
        (self.size() * 4, self.size() * 2)
    }

    fn grid_mids(&self) -> (usize, usize) {
        let (x, y) = self.grid_dimensions();
        (x / 2, y / 2)
    }

    fn rotate(&self, id: usize, owner: &Owner) -> (usize, Option<usize>) {
        let (x, y) = self.get_local_xy(id).expect("Id not found");
        let map = self.rotation_map.get(owner).expect("Owner not found");
        let rotated = map[y][x];
        let (xp, yp) = self.get_local_xy(rotated).expect("Id not found");
        let parent = match owner {
            Owner::None => unimplemented!(),
            Owner::Own => Some((xp, yp)),
            Owner::Foe1 => {
                if yp > 0 {
                    if yp == self.size && xp > 0 {
                        Some((xp - 1, yp - 1))
                    } else {
                        Some((xp, yp - 1))
                    }
                } else {
                    None
                }
            }
            Owner::Foe2 => {
                if xp > 1 {
                    Some((xp - 2, yp))
                } else {
                    None
                }
            }
        };
        println!("  Parent:{:?} x,y :{xp}, {yp}", parent);
        let parent: Option<usize> = parent.map(|(x, y)| self.hexagon[y][x]);

        (rotated, parent)
    }
}

impl From<usize> for Hexagon {
    fn from(size: usize) -> Self {
        let mut hexagon: Vec<Vec<usize>> = vec![vec![]; 2 * size];
        let (x_size, y_size) = (size * 4, size * 2);
        for y in 0..y_size {
            for x in 0..x_size {
                match get_id(size, x, y) {
                    Some(id) => hexagon[y].push(id),
                    None => continue,
                }
            }
        }

        let rotation1 = Hexagon::rotate_hexagon(&hexagon, size);
        let rotation2 = Hexagon::rotate_hexagon(&rotation1, size);

        let rotation_map = HashMap::from([
            (Owner::Own, hexagon.clone()),
            (Owner::Foe1, rotation1),
            (Owner::Foe2, rotation2),
        ]);

        Self {
            hexagon,
            size,
            rotation_map,
        }
    }
}

fn get_id(size: usize, x: usize, y: usize) -> Option<usize> {
    if x >= size * 4 || y >= size * 2 || front_skip(x, y, size) || back_skip(x, y, size) {
        return None;
    }

    let raw = y * size * 4 + x;

    let gap = if y < size {
        (y + 1) * (2 * size - y - 1)
    } else {
        size * size + (y - size) * (y - size)
    };

    Some(raw - gap)
}

fn front_skip(x: usize, y: usize, size: usize) -> bool {
    size > y && x < 2 * (size - y) - 1
}

fn back_skip(x: usize, y: usize, size: usize) -> bool {
    y >= size && x > 6 * size - 2 * y - 2
}
