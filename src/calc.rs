use std::sync::Arc;

use cgmath::{ElementWise, InnerSpace, Vector3};
use rand::Rng;

use crate::{hitable::Hitable, ray::Ray};

pub type Color = Vector3<f64>;

pub fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> { v - n * v.dot(n) * 2.0 }

pub fn refract(v: Vector3<f64>, n: Vector3<f64>, index: f64) -> Option<Vector3<f64>> {
    let dt = v.dot(n);
    let discriminant = 1.0 - index * index * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((v - n * dt) * index - n * discriminant.sqrt())
    } else {
        None
    }
}

pub fn schlick(cosine: f64, index: f64) -> f64 {
    let r0 = (1.0 - index) / (1.0 + index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

pub fn color(ray: Ray, env: &Arc<Hitable>, depth: i64) -> Color {
    if let Some(rec) = env.hit(&ray, 0.000_001..std::f64::MAX) {
        if let Some((attenuation, scattered)) = rec.clone().material.scatter(ray, rec) {
            if depth < 50 {
                color(scattered, env, depth + 1).mul_element_wise(attenuation)
            } else {
                Vector3::new(0.0, 0.0, 0.0)
            }
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        }
    } else {
        let t = 0.5 * (ray.direction().y + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

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
