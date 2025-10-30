mod hittable_list;
mod sphere;

use std::sync::Arc;

pub use hittable_list::HittableList;
pub use sphere::Sphere;

use crate::{Interval, Point3, Ray, Vec3};

/// Trait for objects that can be hit by rays.
pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}

/// Different types of hittable objects.
#[derive(Debug, Clone)]
pub enum HittableObject {
    List(HittableList),
    Sphere(Sphere),
}

impl HittableObject {
    pub fn new_list(objects: Vec<Arc<HittableObject>>) -> Self {
        Self::List(HittableList::from_hittables(objects))
    }

    pub fn add(&mut self, hittable: Arc<HittableObject>) {
        match self {
            Self::List(list) => list.add(hittable),
            _ => panic!("Unsupported 'add' operation"),
        }
    }

    pub fn new_sphere(static_center: Point3, radius: f64) -> Self {
        Self::Sphere(Sphere::new(static_center, radius))
    }
}

impl Hittable for HittableObject {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        match self {
            Self::List(list) => list.hit(ray, ray_t),
            Self::Sphere(sphere) => sphere.hit(ray, ray_t),
        }
    }
}

#[derive(Default)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    /// 为 false 则光线位于对象内部，为 true 则光线位于对象外部。
    pub front_face: bool,
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

    /// Sets the face normal based on the given ray and outward normal.
    ///
    /// # Note
    ///
    /// the parameter `outward_normal` is assumed to have unit length.
    pub fn set_face_normal(mut self, r: &Ray, outward_normal: Vec3) -> Self {
        self.front_face = r.direction.dot(&outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };

        self
    }
}
