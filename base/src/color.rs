use std::sync::LazyLock;

use crate::{Color, interval::Interval};

static INTENSITY: LazyLock<Interval> = LazyLock::new(|| Interval::new(0., 0.999));

/// Translate a color into a tuple of bytes
pub fn translate_color(pixel_color: Color) -> (u8, u8, u8) {
    let (r, g, b) = pixel_color.into();

    // translate the [0, 1] component values to the byte range [0, 255]
    (
        // Apply a linear to gamma transform for gamma 2
        (255. * INTENSITY.clamp(linear_to_gamma(r))) as u8,
        (255. * INTENSITY.clamp(linear_to_gamma(g))) as u8,
        (255. * INTENSITY.clamp(linear_to_gamma(b))) as u8,
    )
}

/// Convert a linear component to a gamma component
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0. {
        linear_component.sqrt()
    } else {
        0.
    }
}
