use crate::game::Owner;

use super::ShapeTrait;

#[derive(Clone)]
pub struct Square {
    pub hexagon: Vec<Vec<usize>>,
    pub size: usize,
}

impl From<usize> for Square {
    fn from(size: usize) -> Self {
        let mut hexagon = vec![vec![]; size];
        for y in 0..size {
            for x in 0..size {
                hexagon[y].push(x);
            }
        }
        Self { hexagon, size }
    }
}

impl ShapeTrait for Square {
    fn get_id(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.size || y >= self.size {
            return None;
        }
        Some(x + y * self.size)
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

    fn goal(&self, _: usize, y: usize) -> Owner {
        match y {
            0 => Owner::Own,
            val if val == self.grid_dimensions().1 - 1 => Owner::Foe1,
            _ => Owner::None,
        }
    }

    fn grid_dimensions(&self) -> (usize, usize) {
        (self.size, self.size)
    }
    fn grid_mids(&self) -> (usize, usize) {
        let (x, y) = self.grid_dimensions();
        (x / 2, y / 2)
    }

    fn rotate(&self, id: usize, owner: &Owner) -> (usize, Option<usize>) {
        match owner {
            Owner::None => unimplemented!(),
            Owner::Own => (id, Some(id)),
            Owner::Foe1 => {
                let own = self.size * self.size - 1 - id;
                let parent = if own >= self.size && own - self.size > 0 {
                    Some(own - self.size - 1)
                } else {
                    None
                };
                (own, parent)
            }
            Owner::Foe2 => unreachable!(),
        }
    }
}
