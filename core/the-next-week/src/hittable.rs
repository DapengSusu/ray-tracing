mod bvh;
mod hittable_list;
mod sphere;

pub use bvh::BvhNode;
pub use hittable_list::HittableList;
pub use sphere::Sphere;

use std::sync::Arc;

use crate::prelude::*;

#[derive(Default)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Option<Arc<dyn Material>>,
}

impl HitRecord {
    /// Builds a default `HitRecord`.
    pub fn builder() -> Self {
        Self::default()
    }

    /// Sets the parameter `t` of the hit record.
    pub fn set_t(mut self, t: f64) -> Self {
        self.t = t;
        self
    }

    /// Sets the parameter `p` of the hit record.
    pub fn set_p(mut self, p: Point3) -> Self {
        self.p = p;
        self
    }

    /// Sets the parameter `material` of the hit record.
    pub fn set_material(mut self, material: Option<Arc<dyn Material>>) -> Self {
        self.material = material;
        self
    }

    /// Sets the face normal based on the given ray and outward normal.
    ///
    /// # Note
    ///
    /// the parameter `outward_normal` is assumed to have unit length.
    pub fn set_face_normal(mut self, ray: &Ray, outward_normal: Vec3) -> Self {
        self.front_face = ray.direction.dot(&outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
        self
    }
}

/// Trait for objects that can be hit by rays.
pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> &AABB;
}
