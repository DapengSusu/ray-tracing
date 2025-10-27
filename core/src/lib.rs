pub mod aabb;
pub mod camera;
pub mod color;
pub mod common;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod perlin;
pub mod plane;
pub mod ray;
pub mod texture;
pub mod vec3;

pub use vec3::Vec3 as Point3;
pub use vec3::Vec3 as Color;
pub use vec3::{Vec3, Vec3Iter};

/// Prelude module for importing commonly used types and traits.
pub mod prelude {
    pub use crate::aabb::AABB;
    pub use crate::camera::Camera;
    pub use crate::color;
    pub use crate::common::{self, Degrees, Radians, UvCoord};
    pub use crate::hittable::quad;
    pub use crate::hittable::{
        BvhNode, HitRecord, Hittable, HittableList, HittableObject, Quad, RotateY, Sphere,
        Translate,
    };
    pub use crate::interval::Interval;
    pub use crate::material::{
        Dielectric, DiffuseLight, Lambertian, Material, MaterialType, Metal,
    };
    pub use crate::perlin::Perlin;
    pub use crate::plane::PlaneFigure;
    pub use crate::ray::Ray;
    pub use crate::texture::{
        CheckerTexture, ImageTexture, NoiseTexture, SolidColor, Texture, TextureType,
    };
    pub use crate::vec3::Vec3 as Point3;
    pub use crate::vec3::Vec3 as Color;
    pub use crate::vec3::{self, Vec3, Vec3Iter};
}
