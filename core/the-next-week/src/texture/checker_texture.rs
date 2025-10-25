use std::sync::Arc;

use crate::prelude::*;

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    /// Create a new checker texture with the given scale and even/odd textures.
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: scale.recip(),
            even,
            odd,
        }
    }

    /// Create a new checker texture with the given scale and two colors.
    pub fn from_colors(scale: f64, c1: Color, c2: Color) -> Self {
        Self::new(
            scale,
            Arc::new(SolidColor::new(c1)),
            Arc::new(SolidColor::new(c2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, uv: &UvCoord, p: &Point3) -> Point3 {
        let x_int = (self.inv_scale * p.x).floor() as i64;
        let y_int = (self.inv_scale * p.y).floor() as i64;
        let z_int = (self.inv_scale * p.z).floor() as i64;

        if (x_int + y_int + z_int) % 2 == 0 {
            self.even.as_ref().value(uv, p)
        } else {
            self.odd.as_ref().value(uv, p)
        }
    }
}
