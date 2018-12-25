use std::{f64, sync::Arc};

use crate::{aabb::AABBBox,
            hitable::{HitRecord, Hitable},
            ray::Ray};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct HitableList {
    hitables: Vec<Arc<Hitable>>,
}

impl HitableList {
    #[allow(dead_code)]
    pub fn new(hitables: Vec<Arc<Hitable>>) -> Self { HitableList { hitables } }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, range: std::ops::Range<f64>) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;
        let mut closest = range.end;
        for hitable in self.hitables.iter() {
            if let Some(hit_record) = hitable.hit(ray, range.start..closest) {
                closest = hit_record.t;
                hit = Some(hit_record);
            }
        }
        hit
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABBBox> {
        if self.hitables.is_empty() {
            None
        } else {
            Some(self.hitables.iter().fold(AABBBox::largest(), move |aabb_box, iter| {
                aabb_box.box_add(iter.bounding_box(t0, t1).unwrap_or_else(AABBBox::largest))
            }))
        }
    }
}
