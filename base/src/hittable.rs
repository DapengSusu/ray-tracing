use crate::{Interval, Point3, Ray, Vec3};

#[derive(Debug, Default)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    /// Builds a default `HitRecord`.
    pub fn build() -> Self {
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
}
