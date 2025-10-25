use crate::{Color, Point3, common::UvCoord, perlin::Perlin, texture::Texture};

/// Texture that takes these floats between 0 and 1 and creates grey colors
#[derive(Default)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::default(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _uv: &UvCoord, p: &Point3) -> Point3 {
        Color::one() * self.noise.noise(self.scale * p)
    }
}
