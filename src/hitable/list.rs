use std::sync::Arc;

use crate::{hitable::{HitRecord, Hitable},
            ray::Ray};

pub struct HitableList {
    hitables: Vec<Arc<Hitable>>,
}

impl HitableList {
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
}
