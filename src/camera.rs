use nalgebra::{Unit, Vector3};

use crate::ray::Ray;

pub struct Camera {
    origin: Vector3<f64>,
    corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            corner: Vector3::new(-2.0, 2.0, -1.0),
            horizontal: Vector3::new(4.0, 0.0, 0.0),
            vertical: Vector3::new(0.0, -4.0, 0.0),
            origin: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, Unit::new_normalize(self.corner + self.horizontal * u + self.vertical * v - self.origin))
    }
}
