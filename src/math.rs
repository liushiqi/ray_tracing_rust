use std::sync::Arc;

use cgmath::{InnerSpace, Vector3};
use rand::Rng;

use crate::{hitable::Object, ray::Ray};

pub type Color = Vector3<f64>;

#[allow(dead_code)]
pub fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> { v - n * v.dot(n) * 2.0 }

#[allow(dead_code)]
pub fn refract(v: Vector3<f64>, n: Vector3<f64>, index: f64) -> Option<Vector3<f64>> {
    let dt = v.dot(n);
    let discriminant = 1.0 - index * index * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((v - n * dt) * index - n * discriminant.sqrt())
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn schlick(cosine: f64, index: f64) -> f64 {
    let r0 = (1.0 - index) / (1.0 + index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

pub fn color(_ray: Ray, _env: &Arc<Object>, _depth: i64) -> Color {
    Vector3::new(0.0, 0.0, 0.0)
}

#[allow(dead_code)]
pub fn random_vector() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    loop {
        let (x, y, z) = rng.gen::<(f64, f64, f64)>();
        let v = Vector3::new(x, y, z) * 2.0 - Vector3::new(1.0, 1.0, 1.0);
        if v.magnitude().sqrt() <= 1.0 {
            break v;
        }
    }
}
