mod dielectric;
mod lambertian;
mod metal;

use std::sync::Arc;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{Color, hittable::HitRecord, ray::Ray, texture::TextureType};

pub trait Material: Sync + Send {
    /// Scatters a ray based on the material properties.
    ///
    /// # Returns
    ///
    /// A tuple containing the attenuation color and the scattered ray.
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
}

/// The type of a material.
#[derive(Debug, Clone)]
pub enum MaterialType {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl MaterialType {
    pub fn new_lamb(texture: TextureType) -> Self {
        MaterialType::Lambertian(Lambertian::new(Arc::new(texture)))
    }

    pub fn new_lamb_from_color(albedo: Color) -> Self {
        MaterialType::Lambertian(Lambertian::from_color(albedo))
    }

    pub fn new_metal(albedo: Color, fuzz: f64) -> Self {
        MaterialType::Metal(Metal::new(albedo, fuzz))
    }

    pub fn new_dielectric(refraction_index: f64) -> Self {
        MaterialType::Dielectric(Dielectric::new(refraction_index))
    }
}

impl Material for MaterialType {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Self::Lambertian(lambertian) => lambertian.scatter(ray_in, hit),
            Self::Metal(metal) => metal.scatter(ray_in, hit),
            Self::Dielectric(dielectric) => dielectric.scatter(ray_in, hit),
        }
    }
}
