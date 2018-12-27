use std::sync::Arc;

use cgmath::Vector3;

use crate::{camera::Camera, hitable::Object};

#[derive(Debug)]
pub struct Scene {
    width: u32,
    height: u32,
    samples: u32,
    camera: Camera,
    world: Arc<Object>,
}

impl Scene {
    #[allow(dead_code)]
    pub fn new(width: u32, height: u32, samples: u32, camera: Camera, world: Arc<Object>) -> Self {
        Scene { width, height, samples, camera, world }
    }

    #[allow(dead_code)]
    pub fn new_random(width: u32, height: u32, samples: u32) -> Self {
        let hitables = Arc::new(crate::hitable::Sphere { center: Vector3::new(0.0, 0.0, 0.0), radius: 1.0 } );
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
            world: hitables,
        }
    }

    pub fn get_width(&self) -> u32 { self.width }

    pub fn get_height(&self) -> u32 { self.height }

    pub fn get_sample_num(&self) -> u32 { self.samples }

    pub fn get_camera(&self) -> &Camera { &self.camera }

    pub fn get_world(&self) -> &Arc<Object> { &self.world }
}
