use std::path::Path;

use crate::prelude::*;

use ray_tracing_extra::mini_stb::LinearPixelImage;

#[derive(Debug, Default, Clone)]
pub struct ImageTexture {
    image: Box<LinearPixelImage>,
}

impl ImageTexture {
    pub fn new<P: AsRef<Path>>(image_path: P) -> Self {
        Self {
            image: Box::new(
                LinearPixelImage::load(image_path.as_ref()).unwrap_or_else(|e| {
                    panic!(
                        "Failed to load image from {}, {e}",
                        image_path.as_ref().display()
                    )
                }),
            ),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, uv: &UvCoord, _p: &Point3) -> Point3 {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.image.height() == 0 {
            return Color::new(0., 1., 1.);
        }

        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = Interval::new(0., 1.).clamp(uv.u);
        // Flip V to image coordinates
        let v = 1. - Interval::new(0., 1.).clamp(uv.v);

        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;
        let pixel = self.image.pixel_data_at(i, j);

        let color_scale = 1. / 255.;

        color_scale * Color::new(pixel.r as f64, pixel.g as f64, pixel.b as f64)
    }
}
