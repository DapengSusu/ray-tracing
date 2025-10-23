mod common;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

/// Prelude module for importing commonly used types and traits.
pub mod prelude {
    pub use crate::common::{Degrees, Radians};
    pub use crate::hittable::{HitRecord, Hittable};
    pub use crate::interval::{EMPTY as INTERVAL_EMPTY, Interval, UNIVERSE as INTERVAL_UNIVERSE};
    pub use crate::ray::Ray;
    pub use crate::vec3::Vec3 as Point3;
    pub use crate::vec3::Vec3 as Color;
    pub use crate::vec3::*;
}

pub use common::{Degrees, Radians};
pub use hittable::{HitRecord, Hittable};
pub use hittable_list::HittableList;
pub use interval::{EMPTY as INTERVAL_EMPTY, Interval, UNIVERSE as INTERVAL_UNIVERSE};
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::Vec3 as Point3;
pub use vec3::Vec3 as Color;
pub use vec3::*;

#[cfg(test)]
mod tests {}
