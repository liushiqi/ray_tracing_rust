use std::sync::Arc;

use cgmath::{InnerSpace, Vector3};

use crate::{hitable::{HitRecord, Hitable},
            material::Material,
            ray::Ray};

#[derive(Debug)]
pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    material: Arc<Material>,
}

impl Sphere {
    pub fn new_from_vec(center: Vector3<f64>, radius: f64, material: Arc<Material>) -> Self {
        Sphere { center, radius, material }
    }

    pub fn new_from_f64(x: f64, y: f64, z: f64, radius: f64, material: Arc<Material>) -> Self {
        Sphere { center: Vector3::new(x, y, z), radius, material }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, range: std::ops::Range<f64>) -> Option<HitRecord> {
        let oc = self.center - ray.origin();
        let a = ray.direction().dot(*ray.direction());
        let b = -oc.dot(*ray.direction()) * 2.0;
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / (2.0 * a);
            if range.contains(&temp) {
                return Some(HitRecord::new(
                    temp,
                    ray.point_at(temp),
                    ((ray.point_at(temp) - self.center) / self.radius).normalize(),
                    Arc::clone(&self.material),
                ));
            }
            let temp = (-b + discriminant.sqrt()) / (2.0 * a);
            if range.contains(&temp) {
                return Some(HitRecord::new(
                    temp,
                    ray.point_at(temp),
                    ((ray.point_at(temp) - self.center) / self.radius).normalize(),
                    Arc::clone(&self.material),
                ));
            }
        }
        None
    }
}
