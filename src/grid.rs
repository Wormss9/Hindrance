use bevy::ecs::resource::Resource;

use crate::game_logic::PlayerLocation;

#[derive(Resource)]
pub struct Edges {
    size: usize,
    shape: Shape,
    walls: Vec<bool>,
    pub edges: Vec<Vec<usize>>,
}

enum Shape {
    Square,
    Triangle,
}

impl Edges {
    pub fn square(size: usize) -> Self {
        let max = size * size;

        let mut edges = vec![Vec::with_capacity(4); max];

        for i in 0..size {
            for j in 0..size {
                let id = i + j * size;
                if j > 0 {
                    edges[id].push(id - size);
                }
                if i > 0 {
                    edges[id].push(id - 1);
                }
                if i + 1 < size {
                    edges[id].push(id + 1);
                }
                if j + 1 < size {
                    edges[id].push(id + size);
                }
            }
        }

        let walls = Vec::with_capacity((size - 1) * (size - 1) * 2);

        Self {
            edges,
            size,
            walls,
            shape: Shape::Square,
        }
    }
    pub fn triangle_hex(size: usize) -> Self {
        let max = 6 * size * size;
        let mut edges = vec![Vec::with_capacity(3); max];
        for y in 0..size * 2 {
            for x in 0..size * 4 {
                match get_id(x, y, size) {
                    Some(current) => {
                        if x > 0
                            && let Some(right) = get_id(x - 1, y, size)
                        {
                            edges[current].push(right);
                        }
                        if x < size * 4 - 1
                            && let Some(left) = get_id(x + 1, y, size)
                        {
                            edges[current].push(left);
                        }
                        if x % 2 == 0 {
                            if y > 0
                                && let Some(top) =
                                    get_id(if y < size { x - 1 } else { x + 1 }, y - 1, size)
                            {
                                edges[current].push(top);
                            }
                        } else if y < size * 2
                            && let Some(bottom) =
                                get_id(if y < size { x - 1 } else { x + 1 }, y + 1, size)
                        {
                            edges[current].push(bottom);
                        }
                    }
                    None => continue,
                }
            }
        }

        let walls = Vec::with_capacity((size - 1) * (size - 1) * 2);

        Self {
            edges,
            size,
            shape: Shape::Triangle,
            walls,
        }
    }
    pub fn reachable_from(&self, player_location: &PlayerLocation) -> Vec<usize> {
        let location = player_location.x + player_location.y * self.size;
        self.edges[location].clone()
    }
}

fn front_skip(x: usize, y: usize, size: usize) -> bool {
    size > y && x < 2 * (size - y) - 1
}

fn back_skip(x: usize, y: usize, size: usize) -> bool {
    y >= size && x > 6 * size - 2 * y - 2
}

pub fn get_id(x: usize, y: usize, size: usize) -> Option<usize> {
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
