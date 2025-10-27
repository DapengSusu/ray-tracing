use std::sync::Arc;

use crate::{
    Color, Vec3,
    ray::Ray,
    texture::{HitRecord, Material, SolidColor, Texture, TextureType},
};

/// The scattering function of isotropic picks a uniform random direction
#[derive(Debug, Clone)]
pub struct Isotropic {
    texture: Arc<TextureType>,
}

impl Isotropic {
    pub fn new(texture: Arc<TextureType>) -> Self {
        Self { texture }
    }

    pub fn with_color(albedo: Color) -> Self {
        Self {
            texture: Arc::new(TextureType::SolidColor(SolidColor::new(albedo))),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let scattered = Ray::new_with_time(hit.p, Vec3::random_unit_vector(), ray_in.time);
        let attenuation = self.texture.value(&hit.uv, &hit.p);

        Some((attenuation, scattered))
    }
}
