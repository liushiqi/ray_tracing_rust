use std::{fmt::Debug, sync::Arc};

use cgmath::{InnerSpace, Vector3};

use crate::{aabb::AABBBox, material::Material, ray::Ray};

pub mod bvh;
pub mod list;
pub mod sphere;

pub type BvhNode = self::bvh::BvhNode;
pub type Sphere = self::sphere::Sphere;
#[allow(dead_code)]
pub type HitableList = self::list::HitableList;

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub t: f64,
    pub position: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub material: Arc<Material>,
}

impl HitRecord {
    pub fn new(t: f64, position: Vector3<f64>, normal: Vector3<f64>, material: Arc<Material>) -> Self {
        HitRecord { t, position, normal: normal.normalize(), material }
    }
}

pub trait Hitable: Debug + Send + Sync {
    fn hit(&self, ray: &Ray, range: std::ops::Range<f64>) -> Option<HitRecord>;

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABBBox>;
}
