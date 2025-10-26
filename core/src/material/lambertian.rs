use std::sync::Arc;

use crate::prelude::*;

/// Lambertian material
#[derive(Debug, Clone)]
pub struct Lambertian {
    texture: Arc<TextureType>,
}

impl Lambertian {
    /// Create a new Lambertian material with the given texture.
    pub fn new(texture: Arc<TextureType>) -> Self {
        Self { texture }
    }

    /// Create a new Lambertian material with the given albedo color.
    pub fn from_color(albedo: Color) -> Self {
        Self {
            texture: Arc::new(TextureType::SolidColor(SolidColor::new(albedo))),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        let attenuation = self.texture.value(&hit.uv, &hit.p);
        let scattered = Ray::new_with_time(hit.p, scatter_direction, ray_in.time);

        Some((attenuation, scattered))
    }
}
