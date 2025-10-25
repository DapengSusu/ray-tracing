pub use crate::prelude::*;

mod checker_texture;
mod image_texture;
mod noise_texture;
mod solid_color;

pub use checker_texture::CheckerTexture;
pub use image_texture::ImageTexture;
pub use noise_texture::NoiseTexture;
pub use solid_color::SolidColor;

pub trait Texture: Sync + Send {
    /// Return the texture color given the input coordinates.
    fn value(&self, uv: &UvCoord, p: &Point3) -> Color;
}
