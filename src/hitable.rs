use std::sync::Arc;

use nalgebra::{Unit, Vector3};

use crate::{material::Material, ray::Ray};

pub mod list;
pub mod sphere;

pub type Sphere = self::sphere::Sphere;
pub type HitableList = self::list::HitableList;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub position: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub material: Arc<Material>,
}

impl HitRecord {
    pub fn new(t: f64, position: Vector3<f64>, normal: Unit<Vector3<f64>>, material: Arc<Material>) -> Self {
        HitRecord { t, position, normal: *normal.as_ref(), material }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, range: std::ops::Range<f64>) -> Option<HitRecord>;
}
