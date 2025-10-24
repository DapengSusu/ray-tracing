mod camera;
mod ray;

pub mod aabb;
pub mod color;
pub mod common;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod texture;
pub mod vec3;

pub use aabb::AABB;
pub use camera::Camera;
pub use interval::Interval;
pub use ray::Ray;
pub use vec3::Vec3 as Point3;
pub use vec3::Vec3 as Color;
pub use vec3::{Vec3, Vec3Iter};

/// Prelude module for importing commonly used types and traits.
pub mod prelude {
    pub use crate::aabb::AABB;
    pub use crate::camera::Camera;
    pub use crate::color;
    pub use crate::common::{self, Degrees, Radians};
    pub use crate::hittable::{BvhNode, HitRecord, Hittable, HittableList, Sphere};
    pub use crate::interval::Interval;
    pub use crate::material::{Dielectric, Lambertian, Material, Metal};
    pub use crate::ray::Ray;
    pub use crate::texture::{CheckerTexture, SolidColor, Texture};
    pub use crate::vec3::Vec3 as Point3;
    pub use crate::vec3::Vec3 as Color;
    pub use crate::vec3::{self, Vec3, Vec3Iter};
}
