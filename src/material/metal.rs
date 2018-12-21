use nalgebra::{Unit, Vector3};

use crate::{calc::{random_vector, reflect, Color},
            hitable::HitRecord,
            material::Material,
            ray::Ray};

#[derive(Clone, Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(r: f64, g: f64, b: f64, fuzz: f64) -> Self { Metal { albedo: Vector3::new(r, g, b), fuzz } }
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, record: HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(Unit::new_normalize(*ray_in.direction()), Unit::new_normalize(record.normal));
        if reflected.dot(&record.normal) > 0.0 {
            Some((self.albedo, Ray::new(record.position, Unit::new_normalize(reflected + self.fuzz * random_vector()))))
        } else {
            None
        }
    }
}
