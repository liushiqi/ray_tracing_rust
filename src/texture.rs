use std::fmt::Debug;

use cgmath::Vector3;

use crate::calc::Color;

pub mod checker;
pub mod constant;

pub type CheckerTexture = self::checker::CheckerTexture;
pub type ConstantTexture = self::constant::ConstantTexture;

pub trait Texture: Debug + Sync + Send {
    fn uv_coordinate(&self, u: f64, v: f64, p: Vector3<f64>) -> Color;
}
