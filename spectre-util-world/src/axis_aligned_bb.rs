use derive_more::{Add, Div, From, Mul, Sub};
use glam::DVec3;

#[derive(Add, Sub, Mul, Div, From)]
struct AxisAlignedBB {
    min_x: f64,
    min_y: f64,
    min_z: f64,
    max_x: f64,
    max_y: f64,
    max_z: f64,
}

impl From<(DVec3, DVec3)> for AxisAlignedBB {
    fn from((vec1, vec2): (DVec3, DVec3)) -> Self {
        AxisAlignedBB {
            min_x: vec1.x,
            min_y: vec1.y,
            min_z: vec1.z,
            max_x: vec2.x,
            max_y: vec2.y,
            max_z: vec2.z,
        }
    }
}
