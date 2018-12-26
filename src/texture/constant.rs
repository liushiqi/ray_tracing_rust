use cgmath::Vector3;

use crate::{calc::Color, texture::Texture};

#[derive(Clone, Debug)]
pub struct ConstantTexture {
    color: Color,
}

impl ConstantTexture {
    pub fn new(r: f64, g: f64, b: f64) -> Self { ConstantTexture { color: Color::new(r, g, b) } }
}

impl Texture for ConstantTexture {
    fn uv_coordinate(&self, _u: f64, _v: f64, _p: Vector3<f64>) -> Color { self.color }
}
