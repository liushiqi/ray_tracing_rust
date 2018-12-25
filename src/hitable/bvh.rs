use std::sync::Arc;

use crate::{aabb::AABBBox,
            hitable::{HitRecord, Hitable},
            ray::Ray};

#[derive(Debug)]
pub struct BvhNode {
    left: Arc<Hitable>,
    right: Arc<Hitable>,
    aabb_box: AABBBox,
}

impl BvhNode {
    pub fn new(hitables: &mut Vec<Arc<Hitable>>, time0: f64, time1: f64) -> Self {
        let axis = (3.0 * rand::random::<f64>()) as usize;
        hitables.sort_by(|hit1, hit2| -> std::cmp::Ordering {
            if let Some(box1) = hit1.bounding_box(0.0, 0.0) {
                if let Some(box2) = hit2.bounding_box(0.0, 0.0) {
                    box1.min()[axis].partial_cmp(&box2.min()[axis]).unwrap()
                } else {
                    panic!("No bounding box for {:?}", hit2)
                }
            } else {
                panic!("No bounding box for {:?}", hit1)
            }
        });
        let (left, right): (Arc<Hitable>, Arc<Hitable>) = if hitables.len() == 1 {
            (hitables[0].clone(), hitables[0].clone())
        } else if hitables.len() == 2 {
            (hitables[0].clone(), hitables[1].clone())
        } else {
            let len = hitables.len();
            let (left, right) = hitables.split_at_mut(len / 2);
            (
                Arc::new(BvhNode::new(&mut Vec::from(left), time0, time1)),
                Arc::new(BvhNode::new(&mut Vec::from(right), time0, time1)),
            )
        };
        let aabb_box = if let Some(box1) = left.bounding_box(0.0, 0.0) {
            if let Some(box2) = right.bounding_box(0.0, 0.0) {
                box1.box_add(box2)
            } else {
                panic!("No bounding box for {:?}", right)
            }
        } else {
            panic!("No bounding box for {:?}", left)
        };
        BvhNode { left, right, aabb_box }
    }
}

impl Hitable for BvhNode {
    fn hit(&self, ray: &Ray, range: std::ops::Range<f64>) -> Option<HitRecord> {
        if self.aabb_box.hit(ray, range.clone()) {
            if let Some(hit_left) = self.left.hit(ray, range.clone()) {
                if let Some(hit_right) = self.right.hit(ray, range) {
                    if hit_left.t < hit_right.t {
                        Some(hit_left)
                    } else {
                        Some(hit_right)
                    }
                } else {
                    Some(hit_left)
                }
            } else if let Some(hit_right) = self.right.hit(ray, range) {
                Some(hit_right)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABBBox> { Some(self.aabb_box.clone()) }
}
