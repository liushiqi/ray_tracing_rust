use nalgebra::{Unit, Vector3};

use crate::ray::Ray;

pub struct Camera {
    origin: Vector3<f64>,
    corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
}

impl Camera {
    pub fn new(look_from: Vector3<f64>, look_at: Vector3<f64>, up: Vector3<f64>, vertical: f64, aspect: f64) -> Self {
        let theta = vertical * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);
        Camera {
            corner: look_from - half_width * u + half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: -2.0 * half_height * v,
            origin: look_from,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, Unit::new_normalize(self.corner + self.horizontal * u + self.vertical * v - self.origin))
    }
}
