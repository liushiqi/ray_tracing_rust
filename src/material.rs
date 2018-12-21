use std::fmt::Debug;

use crate::{calc::Color, hitable::HitRecord, ray::Ray};

pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub type Lambertian = self::lambertian::Lambertian;
pub type Metal = self::metal::Metal;
pub type Dielectric = self::dielectric::Dielectric;

pub trait Material: Debug {
    fn scatter(&self, ray_in: Ray, record: HitRecord) -> Option<(Color, Ray)>;
}
