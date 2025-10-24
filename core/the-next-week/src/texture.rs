pub use crate::prelude::*;

mod checker_texture;
mod solid_color;

pub use checker_texture::CheckerTexture;
pub use solid_color::SolidColor;

pub trait Texture: Sync + Send {
    /// Return the texture color given the input coordinates.
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
