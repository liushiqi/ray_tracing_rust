use std::fmt::Debug;

use cgmath::Vector3;

use crate::math::Color;
use crate::ray::Ray;

pub trait Object: Debug + Send + Sync {
    fn hit(&self, ray: &Ray, range: std::ops::Range<f64>) -> Option<Color>;
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
}

impl Object for Sphere {
    fn hit(&self, _ray: &Ray, _range: std::ops::Range<f64>) -> Option<Color> {
        Some(Color::new(1.0, 1.0, 1.0))
    }
}