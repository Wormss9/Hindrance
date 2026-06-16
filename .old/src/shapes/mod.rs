use enum_dispatch::enum_dispatch;
pub use hexagon::Hexagon;
use serde::{Deserialize, Serialize};
pub use square::Square;

use crate::game::Owner;

mod hexagon;
mod square;

#[enum_dispatch(ShapeTrait)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Shape {
    Square(Square),
    Hexagon(Hexagon),
}

#[enum_dispatch]
pub trait ShapeTrait {
    fn size(&self) -> usize;
    fn get_id(&self, x: usize, y: usize) -> Option<usize>;
    fn get_local_xy(&self, id: usize) -> Option<(usize, usize)>;
    fn goal(&self, x: usize, y: usize) -> Owner;
    fn grid_dimensions(&self) -> (usize, usize);
    fn grid_mids(&self) -> (usize, usize);
    fn rotate_tile(&self, id: usize, owner: &Owner) -> (usize, Option<usize>);
}
