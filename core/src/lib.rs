pub mod vec3;

pub use vec3::Vec3 as Point3;
pub use vec3::Vec3 as Color;
pub use vec3::Vec3;

pub mod prelude {
    pub use crate::vec3::Vec3 as Point3;
    pub use crate::vec3::Vec3 as Color;
    pub use crate::vec3::{self, Vec3};
}
