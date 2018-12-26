use cgmath::InnerSpace;
use std::sync::Arc;

use crate::{calc::{random_vector, Color},
            hitable::HitRecord,
            material::Material,
            ray::Ray,
            texture::Texture};

#[derive(Clone, Debug)]
pub struct Lambertian {
    albedo: Arc<Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<Texture>) -> Self { Lambertian { albedo } }
}

impl Material for Lambertian {
    fn scatter(&self, _: Ray, record: HitRecord) -> Option<(Color, Ray)> {
        let target = record.position + record.normal + random_vector();
        Some((
            self.albedo.uv_coordinate(0.0, 0.0, record.position),
            Ray::new(record.position, (target - record.position).normalize()),
        ))
    }
}
