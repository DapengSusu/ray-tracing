use crate::{Point3, Vec3};

/// A ray with the origin and direction.
#[derive(Debug, Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    /// Create a new ray with the given origin and direction.
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Calculate the point at distance `t` along the ray.
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
