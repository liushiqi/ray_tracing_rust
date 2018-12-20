#![feature(uniform_paths)]
#![feature(box_patterns)]
#![feature(range_contains)]

use std::sync::Arc;

use image::{ImageBuffer, Rgb};

use calc::{color, Color};
use camera::Camera;
use hitable::{HitableList, Sphere};
use material::{Lambertian, Metal, Dielectric};

mod calc;
mod camera;
mod hitable;
mod material;
mod ray;

const IMAGE_SIZE: u32 = 1024;
const SAMPLES: u32 = 256;

fn main() {
    let camera = Camera::new();
    let sphere1 = Sphere::new(0.0, 0.0, -1.0, 0.5, Arc::new(Lambertian::new(0.8, 0.3, 0.3)));
    let sphere2 = Sphere::new(0.0, -100.5, -1.0, 100.0, Arc::new(Lambertian::new(0.8, 0.8, 0.0)));
    let sphere3 = Sphere::new(1.0, 0.0, -1.0, 0.5, Arc::new(Metal::new(0.8, 0.6, 0.2, 0.0)));
    let sphere4 = Sphere::new(-1.0, 0.0, -1.0, 0.5, Arc::new(Dielectric::new(1.5)));
    let sphere5 = Sphere::new(-1.0, 0.0, -1.0, -0.45, Arc::new(Dielectric::new(1.5)));
    let hitables =
        HitableList::new(vec![Arc::from(sphere1), Arc::from(sphere2), Arc::from(sphere3), Arc::from(sphere4), Arc::from(sphere5)]);
    ImageBuffer::from_fn(IMAGE_SIZE, IMAGE_SIZE, |x, y| {
        let mut col = Color::new(0.0, 0.0, 0.0);
        for _ in 0..SAMPLES {
            let u = x as f64 / IMAGE_SIZE as f64;
            let v = y as f64 / IMAGE_SIZE as f64;
            col += color(camera.get_ray(u, v), &hitables, 0);
        }
        col /= SAMPLES as f64;
        Rgb([(col.x.sqrt() * 255.0) as u8, (col.y.sqrt() * 255.0) as u8, (col.z.sqrt() * 255.0) as u8])
    })
        .save("render.png")
        .unwrap();
}
