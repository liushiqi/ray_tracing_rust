use nalgebra::{Unit, Vector3};

#[derive(Debug)]
pub struct Ray {
    from: Vector3<f64>,
    to: Vector3<f64>,
}

impl Ray {
    pub fn new(orig: Vector3<f64>, dir: Unit<Vector3<f64>>) -> Ray { Ray { from: orig, to: *dir.as_ref() } }

    pub fn origin(&self) -> &Vector3<f64> { &self.from }

    pub fn direction(&self) -> &Vector3<f64> { &self.to }

    pub fn point_at(&self, t: f64) -> Vector3<f64> { self.from + t * self.to }
}
