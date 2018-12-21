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

const IMAGE_WIDTH: u32 = 1920;
const IMAGE_HEIGHT: u32 = 1080;
// 16:  About 7.345  min
// 64:  About 12.762 min
// 256: About 49.687 min
const SAMPLES: u32 = 64;

fn main() {
    let camera = Camera::new(
        Vector3::new(-6.0, 4.0, 1.0),
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
        90.0,
        IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64,
    );
    let scene = random_scene();
    ImageBuffer::from_fn(IMAGE_WIDTH, IMAGE_HEIGHT, |x, y| {
        let mut col = Color::new(0.0, 0.0, 0.0);
        for _ in 0..SAMPLES {
            let u = x as f64 / IMAGE_WIDTH as f64;
            let v = y as f64 / IMAGE_HEIGHT as f64;
            col += color(camera.get_ray(u, v), &scene, 0);
        }
        if x == IMAGE_WIDTH - 1 {
            println!("Finished rate: {}", (x + y * IMAGE_WIDTH) as f64 / ((IMAGE_WIDTH * IMAGE_HEIGHT) as f64));
        }
        col /= SAMPLES as f64;
        Rgb([(col.x.sqrt() * 255.0) as u8, (col.y.sqrt() * 255.0) as u8, (col.z.sqrt() * 255.0) as u8])
    })
    .save("render.png")
    .unwrap();
}
