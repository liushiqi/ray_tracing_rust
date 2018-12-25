use std::f64;

use cgmath::Vector3;

use crate::ray::Ray;

#[derive(Clone, Debug)]
pub struct AABBBox {
    min: Vector3<f64>,
    max: Vector3<f64>,
}

impl AABBBox {
    pub fn new(min: Vector3<f64>, max: Vector3<f64>) -> Self { AABBBox { min, max } }

    pub fn largest() -> Self {
        AABBBox { min: Vector3::new(f64::MIN, f64::MIN, f64::MIN), max: Vector3::new(f64::MAX, f64::MAX, f64::MAX) }
    }

    pub fn hit(&self, ray: &Ray, range: std::ops::Range<f64>) -> bool {
        let mut start = range.start;
        let mut end = range.end;
        for i in 0..3 {
            if ray.direction()[i] >= 0.0 {
                start = ((self.min[i] - ray.origin()[i]) / ray.direction()[i]).max(start);
                end = ((self.max[i] - ray.origin()[i]) / ray.direction()[i]).min(end);
            } else {
                start = ((self.max[i] - ray.origin()[i]) / ray.direction()[i]).max(start);
                end = ((self.min[i] - ray.origin()[i]) / ray.direction()[i]).min(end);
            }
            if start >= end {
                return false;
            }
        }
        true
    }

    pub fn box_add(&self, aabb_box: AABBBox) -> AABBBox {
        AABBBox {
            min: Vector3::new(
                self.min.x.min(aabb_box.min.x),
                self.min.y.min(aabb_box.min.y),
                self.min.z.min(aabb_box.min.z),
            ),
            max: Vector3::new(
                self.max.x.max(aabb_box.max.x),
                self.max.y.max(aabb_box.max.y),
                self.max.z.max(aabb_box.max.z),
            ),
        }
    }

    pub fn min(&self) -> Vector3<f64> { self.min }
}
