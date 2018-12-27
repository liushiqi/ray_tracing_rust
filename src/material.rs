use std::fmt::Debug;

use cgmath::Vector3;

use crate::{math::Color, ray::Ray};

pub trait Material: Debug + Send + Sync {
    fn scatter(&self, ray_in: Ray) -> Option<(Color, Ray)>;

    fn emitted(&self, _u: f64, _v: f64, _position: Vector3<f64>) -> Vector3<f64> { Vector3::new(0.0, 0.0, 0.0) }
}
