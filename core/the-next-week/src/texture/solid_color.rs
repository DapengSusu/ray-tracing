use crate::prelude::*;

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    /// Create a new solid color texture with the given color.
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    /// Create a new solid color texture with the given RGB values.
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self::new(Color::new(r, g, b))
    }
}

impl Texture for SolidColor {
    fn value(&self, _uv: &UvCoord, _p: &Point3) -> Point3 {
        self.albedo
    }
}
