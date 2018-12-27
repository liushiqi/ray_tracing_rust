use cgmath::{InnerSpace, Vector3};

#[derive(Debug)]
pub struct Ray {
    from: Vector3<f64>,
    to: Vector3<f64>,
}

impl Ray {
    pub fn new(orig: Vector3<f64>, dir: Vector3<f64>) -> Ray { Ray { from: orig, to: dir.normalize() } }

    #[allow(dead_code)]
    pub fn origin(&self) -> &Vector3<f64> { &self.from }

    #[allow(dead_code)]
    pub fn direction(&self) -> &Vector3<f64> { &self.to }

    #[allow(dead_code)]
    pub fn point_at(&self, t: f64) -> Vector3<f64> { self.from + t * self.to }
}
