use std::{path::Path, sync::Arc};

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

#[derive(Debug, Clone)]
pub enum TextureType {
    SolidColor(SolidColor),
    Checker(CheckerTexture),
    Image(ImageTexture),
    Noise(NoiseTexture),
}

impl TextureType {
    pub fn new_solid_from_color(albedo: Color) -> Self {
        TextureType::SolidColor(SolidColor::new(albedo))
    }

    pub fn new_solid_from_rgb(r: f64, g: f64, b: f64) -> Self {
        let albedo = Color::new(r, g, b);

        TextureType::new_solid_from_color(albedo)
    }

    pub fn new_checker(scale: f64, even: TextureType, odd: TextureType) -> Self {
        TextureType::Checker(CheckerTexture::new(scale, Arc::new(even), Arc::new(odd)))
    }

    pub fn new_checker_from_colors(scale: f64, c1: Color, c2: Color) -> Self {
        TextureType::Checker(CheckerTexture::from_colors(scale, c1, c2))
    }

    pub fn new_image<P: AsRef<Path>>(image_path: P) -> Self {
        TextureType::Image(ImageTexture::new(image_path))
    }

    pub fn new_noise(scale: f64) -> Self {
        TextureType::Noise(NoiseTexture::new(scale))
    }
}

impl Texture for TextureType {
    fn value(&self, uv: &UvCoord, p: &Point3) -> Color {
        match self {
            TextureType::Checker(checker) => checker.value(uv, p),
            TextureType::Image(image) => image.value(uv, p),
            TextureType::Noise(noise) => noise.value(uv, p),
            TextureType::SolidColor(solid) => solid.value(uv, p),
        }
    }
}
