use bevy::prelude::*;
use strum::EnumIter;


#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SquareWall {
    Right,
    Down,
}
impl From<SquareWall> for Quat {
    fn from(value: SquareWall) -> Self {
        Quat::from_rotation_z(match value {
            SquareWall::Right => std::f32::consts::FRAC_PI_2,
            SquareWall::Down => 0.,
        })
    }
}

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriangleWall {
    Down,
    UpRight,
    DownRight,
}
impl From<TriangleWall> for Quat {
    fn from(value: TriangleWall) -> Quat {
        Quat::from_rotation_z(match value {
            TriangleWall::Down => 0.,
            TriangleWall::UpRight => -std::f32::consts::FRAC_PI_3,
            TriangleWall::DownRight => std::f32::consts::FRAC_PI_3,
        })
    }
}
