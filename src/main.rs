#![feature(uniform_paths)]
#![feature(box_patterns)]
#![feature(range_contains)]

use clap::{App, Arg, ArgMatches};
use image::{ImageBuffer, Rgb};

use crate::{calc::{color, Color},
            scene::Scene};

mod calc;
mod camera;
mod hitable;
mod material;
mod ray;
mod scene;

const IMAGE_WIDTH: u32 = 2560;
const IMAGE_HEIGHT: u32 = 1440;
const SAMPLES: u32 = 64;

fn main() {
    let args = get_args();
    let scene = Scene::new_random(IMAGE_WIDTH, IMAGE_HEIGHT);
    ImageBuffer::from_fn(IMAGE_WIDTH, IMAGE_HEIGHT, |x, y| {
        let mut col = Color::new(0.0, 0.0, 0.0);
        for _ in 0..SAMPLES {
            let u = f64::from(x) / f64::from(IMAGE_WIDTH);
            let v = f64::from(y) / f64::from(IMAGE_HEIGHT);
            col += color(scene.get_camera().get_ray(u, v), &scene.get_world(), 0);
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

fn get_args<'a>() -> ArgMatches<'a> {
    App::new("rtrs")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("SCENE").long("scene").short("s").help("Scene File").takes_value(true).required(true))
        .get_matches()
}
