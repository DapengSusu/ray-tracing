use crate::{Point3, Vec3};

/// A ray with the origin and direction.
#[derive(Debug, Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    /// Create a new ray with the given origin and direction, default time is zero.
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            time: 0.,
        }
    }

    /// Create a new ray with the given origin, direction, and time.
    pub fn new_with_time(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    /// Calculate the point at distance `t` along the ray.
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
