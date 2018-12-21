#![feature(uniform_paths)]
#![feature(box_patterns)]
#![feature(range_contains)]

use image::{ImageBuffer, Rgb};
use nalgebra::Vector3;

use calc::{color, random_scene, Color};
use camera::Camera;

mod calc;
mod camera;
mod hitable;
mod material;
mod ray;

const IMAGE_WIDTH: u32 = 2560;
const IMAGE_HEIGHT: u32 = 1440;
const SAMPLES: u32 = 64;

fn main() {
    let camera = Camera::new(
        Vector3::new(-6.0, 4.0, 1.0), 
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
        90.0,
        f64::from(IMAGE_WIDTH) / f64::from(IMAGE_HEIGHT),
    );
    let scene = random_scene();
    ImageBuffer::from_fn(IMAGE_WIDTH, IMAGE_HEIGHT, |x, y| {
        let mut col = Color::new(0.0, 0.0, 0.0);
        for _ in 0..SAMPLES {
            let u = f64::from(x) / f64::from(IMAGE_WIDTH);
            let v = f64::from(y) / f64::from(IMAGE_HEIGHT);
            col += color(camera.get_ray(u, v), &scene, 0);
        }
        if x == IMAGE_WIDTH - 1 {
            println!("Finished rate: {}", f64::from(x + y * IMAGE_WIDTH) / f64::from(IMAGE_WIDTH * IMAGE_HEIGHT));
        }
        col /= f64::from(SAMPLES);
        Rgb([(col.x.sqrt() * 255.0) as u8, (col.y.sqrt() * 255.0) as u8, (col.z.sqrt() * 255.0) as u8])
    })
    .save("render.png")
    .unwrap();
}
