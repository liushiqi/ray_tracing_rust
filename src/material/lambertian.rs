use nalgebra::{Unit, Vector3};

use crate::{calc::{random_vector, Color},
            hitable::HitRecord,
            material::Material,
            ray::Ray};

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(r: f64, g: f64, b: f64) -> Self { Lambertian { albedo: Vector3::new(r, g, b) } }
}

impl Material for Lambertian {
    fn scatter(&self, _: Ray, record: HitRecord) -> Option<(Color, Ray)>
    where Self: Sized {
        let target = record.position + record.normal + random_vector();
        Some((self.albedo, Ray::new(record.position, Unit::new_normalize(target - record.position))))
    }
}
