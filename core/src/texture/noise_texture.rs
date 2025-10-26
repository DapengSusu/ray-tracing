use crate::{Color, Point3, common::UvCoord, perlin::Perlin, texture::Texture};

/// Texture that takes these floats between 0 and 1 and creates grey colors
#[derive(Debug, Default, Clone)]
pub struct NoiseTexture {
    noise: Box<Perlin>,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Box::new(Perlin::default()),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _uv: &UvCoord, p: &Point3) -> Point3 {
        Color::new(0.5, 0.5, 0.5)
            * (1. + (self.scale * p.z + 10. * self.noise.turbulence(p, 7)).sin())
    }
}
