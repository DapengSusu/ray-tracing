mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{Color, hittable::HitRecord, ray::Ray};

pub trait Material: Sync + Send {
    /// Scatters a ray based on the material properties.
    ///
    /// # Returns
    ///
    /// A tuple containing the attenuation color and the scattered ray.
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
}
