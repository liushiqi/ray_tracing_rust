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

const IMAGE_WIDTH: u32 = 100;
const IMAGE_HEIGHT: u32 = 100;
const SAMPLES: u32 = 64;

fn main() {
    get_args();
    let scene = Scene::new_random(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES);
    ImageBuffer::from_fn(scene.get_width(), scene.get_height(), |x, y| {
        let mut col = Color::new(0.0, 0.0, 0.0);
        for _ in 0..scene.get_sample_num() {
            let u = f64::from(x) / f64::from(scene.get_width());
            let v = f64::from(y) / f64::from(scene.get_height());
            col += color(scene.get_camera().get_ray(u, v), &scene.get_world(), 0);
        }
        if x == scene.get_width() - 1 {
            println!(
                "Finished rate: {}",
                f64::from(x + y * scene.get_width()) / f64::from(scene.get_width() * scene.get_height())
            );
        }
        col /= f64::from(scene.get_sample_num());
        Rgb([(col.x.sqrt() * 255.0) as u8, (col.y.sqrt() * 255.0) as u8, (col.z.sqrt() * 255.0) as u8])
    })
    .save("temp.png")
    .unwrap();
}

fn get_args<'a>() -> ArgMatches<'a> {
    App::new("rtrs")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("SCENE").long("scene").short("s").help("Scene File (TODO)").takes_value(true))//.required(true))
        .get_matches()
}
