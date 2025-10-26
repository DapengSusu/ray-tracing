mod dielectric;
mod diffuse_light;
mod lambertian;
mod metal;

use std::sync::Arc;

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{Color, Point3, common::UvCoord, hittable::HitRecord, ray::Ray, texture::TextureType};

pub trait Material: Sync + Send {
    /// Scatters a ray based on the material properties.
    ///
    /// # Returns
    ///
    /// A tuple containing the attenuation color and the scattered ray.
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let _ = ray_in;
        let _ = hit;

        None
    }

    /// Like the background, it just tells the ray what color it is and performs no reflection.
    fn emitted(&self, uv: &UvCoord, p: &Point3) -> Color {
        let _ = uv;
        let _ = p;

        Color::zero()
    }
}

/// The type of a material.
#[derive(Debug, Clone)]
pub enum MaterialType {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
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

    pub fn new_diff_light(texture: TextureType) -> Self {
        MaterialType::DiffuseLight(DiffuseLight::new(Arc::new(texture)))
    }

    pub fn new_diff_light_from_color(emit: Color) -> Self {
        MaterialType::DiffuseLight(DiffuseLight::from_color(emit))
    }
}

impl Material for MaterialType {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Self::Lambertian(lambertian) => lambertian.scatter(ray_in, hit),
            Self::Metal(metal) => metal.scatter(ray_in, hit),
            Self::Dielectric(dielectric) => dielectric.scatter(ray_in, hit),
            Self::DiffuseLight(diffuse_light) => diffuse_light.scatter(ray_in, hit),
        }
    }

    fn emitted(&self, uv: &UvCoord, p: &Color) -> Color {
        match self {
            Self::Lambertian(lambertian) => lambertian.emitted(uv, p),
            Self::Metal(metal) => metal.emitted(uv, p),
            Self::Dielectric(dielectric) => dielectric.emitted(uv, p),
            Self::DiffuseLight(diffuse_light) => diffuse_light.emitted(uv, p),
        }
    }
}
