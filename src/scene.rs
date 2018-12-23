use std::sync::Arc;

use cgmath::{InnerSpace, Vector3};
use rand::Rng;

use crate::{camera::Camera,
            hitable::{Hitable, HitableList, Sphere},
            material::{Dielectric, Lambertian, Metal}};

#[derive(Debug)]
pub struct Scene {
    width: u32,
    height: u32,
    samples: u32,
    camera: Camera,
    world: Arc<Hitable>,
}

impl Scene {
    #[allow(dead_code)]
    pub fn new(width: u32, height: u32, samples: u32, camera: Camera, world: Arc<Hitable>) -> Self {
        Scene { width, height, samples, camera, world }
    }

    pub fn new_random(width: u32, height: u32, samples: u32) -> Self {
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
                let center =
                    Vector3::new(f64::from(a) + 0.9 * rng.gen::<f64>(), 0.2, f64::from(b) + 0.9 * rng.gen::<f64>());
                if (center - Vector3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
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
        Scene {
            width,
            height,
            samples,
            camera: Camera::new(
                Vector3::new(-6.0, 4.0, 1.0),
                Vector3::new(0.0, 0.0, -1.0),
                Vector3::new(0.0, 1.0, 0.0),
                90.0,
                f64::from(width) / f64::from(height),
            ),
            world: Arc::from(HitableList::new(hitables)),
        }
    }

    pub fn get_width(&self) -> u32 { self.width }

    pub fn get_height(&self) -> u32 { self.height }

    pub fn get_sample_num(&self) -> u32 { self.samples }

    pub fn get_camera(&self) -> &Camera { &self.camera }

    pub fn get_world(&self) -> &Arc<Hitable> { &self.world }
}
