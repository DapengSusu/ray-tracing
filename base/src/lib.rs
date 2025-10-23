mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

pub mod common;
pub mod interval;

pub use crate::vec3::*;
pub use vec3::Vec3 as Point3;
pub use vec3::Vec3 as Color;

/// Prelude module for importing commonly used types and traits.
pub mod prelude {
    pub use crate::hittable::{HitRecord, Hittable};
    pub use crate::interval::Interval;
    pub use crate::ray::Ray;
    pub use crate::vec3::Vec3 as Point3;
    pub use crate::vec3::Vec3 as Color;
    pub use crate::vec3::*;
}

pub use camera::*;
pub use hittable::{HitRecord, Hittable};
pub use hittable_list::HittableList;
pub use sphere::Sphere;

#[cfg(test)]
mod tests {}
