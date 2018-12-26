use std::sync::Arc;

use cgmath::Vector3;

use crate::{calc::Color, texture::Texture};

#[derive(Debug)]
pub struct CheckerTexture {
    odd: Arc<Texture>,
    even: Arc<Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Arc<Texture>, even: Arc<Texture>) -> Self { CheckerTexture { odd, even } }
}

impl Texture for CheckerTexture {
    fn uv_coordinate(&self, u: f64, v: f64, p: Vector3<f64>) -> Color {
        let sin = (p.x * 10.0).sin() * (p.y * 10.0).sin() * (p.z * 10.0).sin();
        if sin < 0.0 {
            self.odd.uv_coordinate(u, v, p)
        } else {
            self.even.uv_coordinate(u, v, p)
        }
    }
}
