use enum_dispatch::enum_dispatch;
pub use hexagon::Hexagon;
pub use square::Square;

use crate::game::Owner;

mod hexagon;
mod square;

#[enum_dispatch(ShapeTrait)]
#[derive(Clone)]
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
    fn rotate(&self, id: usize, owner: &Owner) -> (usize, Option<usize>);
}
