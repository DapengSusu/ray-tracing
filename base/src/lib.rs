mod ray;
mod vec3;

pub use ray::Ray;
pub use vec3::Vec3 as Point3;
pub use vec3::Vec3 as Color;
pub use vec3::*;

#[cfg(test)]
mod tests {}
