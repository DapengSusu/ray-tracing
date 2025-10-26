use std::sync::Arc;

use crate::{
    Color, Point3,
    common::UvCoord,
    texture::{Material, SolidColor, Texture, TextureType},
};

/// 发光材料
#[derive(Debug, Clone)]
pub struct DiffuseLight {
    texture: Arc<TextureType>,
}

impl DiffuseLight {
    /// Create a new DiffuseLight material with the given texture.
    pub fn new(texture: Arc<TextureType>) -> Self {
        Self { texture }
    }

    /// Create a new DiffuseLight material with the given emited color.
    pub fn from_color(emit: Color) -> Self {
        Self {
            texture: Arc::new(TextureType::SolidColor(SolidColor::new(emit))),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, uv: &UvCoord, p: &Point3) -> Point3 {
        self.texture.value(uv, p)
    }
}
