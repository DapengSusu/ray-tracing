pub mod color;
pub use color::{COLOR_BLACK, COLOR_WHITE};

mod interval;
pub use interval::{INTERVAL_EMPTY, INTERVAL_UNIVERSE, Interval};

mod pnm_image;
pub use pnm_image::{PixelProcessor, PnmFormat, PnmImage, Rgb};

pub mod vec3;
pub use vec3::Vec3 as Point3;
pub use vec3::Vec3 as Color;
pub use vec3::Vec3;
