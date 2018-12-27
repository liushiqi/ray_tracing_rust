use std::fmt::Debug;

use cgmath::Vector3;

use crate::math::Color;

pub trait Texture: Debug + Sync + Send {
    fn uv_coordinate(&self, u: f64, v: f64, position: Vector3<f64>) -> Color;
}
