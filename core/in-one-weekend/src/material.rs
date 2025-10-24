mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{Color, hittable::HitRecord, ray::Ray};

pub trait Material: Sync + Send {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
}
