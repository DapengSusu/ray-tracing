pub mod color;
pub use color::{COLOR_BLACK, COLOR_WHITE, Color};

pub mod common;
pub use common::UvCoord;

mod hittable;
pub use hittable::{HitRecord, Hittable, HittableList, HittableObject, Sphere};

mod interval;
pub use interval::{INTERVAL_EMPTY, INTERVAL_UNIVERSE, Interval};

mod pnm_image;
pub use pnm_image::{PnmFormat, PnmImage, Rgb};

mod ray;
pub use ray::Ray;

pub mod vec3;
pub use vec3::Vec3 as Point3;
pub use vec3::Vec3;

pub trait Renderer: Send + Sync {
    fn render(&self, i: u32, j: u32) -> Rgb;
}
