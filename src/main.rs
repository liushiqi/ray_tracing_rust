#![feature(uniform_paths)]
#![feature(box_patterns)]
#![feature(range_contains)]

use clap::{App, Arg, ArgMatches};
use image::{ImageBuffer, Rgb};
use rayon::prelude::*;

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
    get_args();
    let scene = Scene::new_random(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES);
    let mut buffer = (0..(scene.get_height() * scene.get_width() * 3))
        .map(|x| {
            if x % 3 == 0 {
                ((x / 3) % 256) as u8
            } else if x % 3 == 1 {
                (((x - 1) / 3 / 256) % 256) as u8
            } else {
                (((x - 2) / 3 / 256 / 256) % 256) as u8
            }
        })
        .collect::<Vec<_>>();
    buffer.par_chunks_mut((scene.get_width() * 3) as usize).for_each(|array| {
        array.chunks_mut(3).for_each(|array| {
            let index = u32::from(array[0]) + u32::from(array[1]) * 256 + u32::from(array[2]) * 256 * 256;
            let x = index % scene.get_width();
            let y = index / scene.get_width();
            let mut col = Color::new(0.0, 0.0, 0.0);
            for _ in 0..scene.get_sample_num() {
                let u = f64::from(x) / f64::from(scene.get_width());
                let v = f64::from(y) / f64::from(scene.get_height());
                col += color(scene.get_camera().get_ray(u, v), &scene.get_world(), 0);
            }
            col /= f64::from(scene.get_sample_num());
            array[0] = (col.x.sqrt() * 255.0) as u8;
            array[1] = (col.y.sqrt() * 255.0) as u8;
            array[2] = (col.z.sqrt() * 255.0) as u8;
        })
    });
    ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(scene.get_width(), scene.get_height(), buffer)
        .unwrap()
        .save("render.png")
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
