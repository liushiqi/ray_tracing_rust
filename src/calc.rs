use std::sync::Arc;

use nalgebra::{Unit, Vector3};
use rand::Rng;

use crate::{hitable::{Hitable, HitableList, Sphere},
            material::{Dielectric, Lambertian, Metal},
            ray::Ray};

pub type Color = Vector3<f64>;

pub fn reflect(v: Unit<Vector3<f64>>, n: Unit<Vector3<f64>>) -> Vector3<f64> {
    v.as_ref() - n.as_ref() * v.dot(n.as_ref()) * 2.0
}

pub fn refract(v: Unit<Vector3<f64>>, n: Unit<Vector3<f64>>, index: f64) -> Option<Vector3<f64>> {
    let dt = v.dot(n.as_ref());
    let discriminant = 1.0 - index * index * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((v.as_ref() - n.as_ref() * dt) * index - n.as_ref() * discriminant.sqrt())
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
    if let Some(rec) = env.hit(&ray, 0.000001..std::f64::MAX) {
        if let Some((attenuation, scattered)) = rec.clone().material.scatter(ray, rec) {
            if depth < 50 {
                color(scattered, env, depth + 1).component_mul(&attenuation)
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
        if v.norm().sqrt() <= 1.0 {
            break v;
        }
    }
}

pub fn random_scene() -> Arc<Hitable> {
    let mut rng = rand::thread_rng();
    let mut hitables: Vec<Arc<Hitable>> = Vec::new();
    hitables.push(Arc::from(Sphere::new_from_f64(
        0.0,
        -1000.0,
        0.0,
        1000.0,
        Arc::from(Lambertian::new(0.5, 0.5, 0.5)),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vector3::new(a as f64 + 0.9 * rng.gen::<f64>(), 0.2, b as f64 + 0.9 * rng.gen::<f64>());
            if (center - Vector3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                hitables.push(if choose_mat < 0.8 {
                    Arc::from(Sphere::new_from_vec(
                        center,
                        0.2,
                        Arc::from(Lambertian::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        )),
                    ))
                } else if choose_mat < 0.95 {
                    Arc::from(Sphere::new_from_vec(
                        center,
                        0.2,
                        Arc::from(Metal::new(
                            (rng.gen::<f64>() + 1.0) * 0.5,
                            (rng.gen::<f64>() + 1.0) * 0.5,
                            (rng.gen::<f64>() + 1.0) * 0.5,
                            rng.gen::<f64>() * 0.5,
                        )),
                    ))
                } else {
                    Arc::from(Sphere::new_from_vec(center, 0.2, Arc::from(Dielectric::new(1.5))))
                })
            }
        }
    }
    hitables.push(Arc::from(Sphere::new_from_f64(0.0, 1.0, 0.0, 1.0, Arc::from(Dielectric::new(1.5)))));
    hitables.push(Arc::from(Sphere::new_from_f64(-4.0, 1.0, 0.0, 1.0, Arc::from(Lambertian::new(0.4, 0.2, 0.1)))));
    hitables.push(Arc::from(Sphere::new_from_f64(4.0, 1.0, 0.0, 1.0, Arc::from(Metal::new(0.7, 0.6, 0.5, 1.0)))));
    Arc::from(HitableList::new(hitables))
}
