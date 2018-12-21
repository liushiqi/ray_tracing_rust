use nalgebra::{Unit, Vector3};
use rand::random;

use crate::{calc::{reflect, refract, schlick, Color},
            hitable::HitRecord,
            material::Material,
            ray::Ray};

#[derive(Clone)]
pub struct Dielectric {
    index: f64,
}

impl Dielectric {
    pub fn new(index: f64) -> Self { Dielectric { index } }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: Ray, record: HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(Unit::new_normalize(*ray_in.direction()), Unit::new_normalize(record.normal));
        let (normal, index, cosine) = if ray_in.direction().dot(&record.normal) > 0.0 {
            (-record.normal, self.index, self.index * ray_in.direction().dot(&record.normal))
        } else {
            (record.normal, 1.0 / self.index, -ray_in.direction().dot(&record.normal))
        };
        let reflect_prob = schlick(cosine, self.index);
        Some(
            if let Some(refracted) =
                refract(Unit::new_normalize(*ray_in.direction()), Unit::new_normalize(normal), index)
            {
                (
                    Vector3::new(1.0, 1.0, 1.0),
                    Ray::new(
                        record.position,
                        Unit::new_normalize(if random::<f64>() < reflect_prob { reflected } else { refracted }),
                    ),
                )
            } else {
                (Vector3::new(1.0, 1.0, 1.0), Ray::new(record.position, Unit::new_normalize(reflected)))
            },
        )
    }
}
