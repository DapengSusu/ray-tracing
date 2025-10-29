// use std::sync::LazyLock;

use crate::{Color, Rgb};

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// pub struct Rgb {
//     pub r: u8,
//     pub g: u8,
//     pub b: u8,
// }

// impl Display for Rgb {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} {} {}", self.r, self.g, self.b)
//     }
// }

/// 白色
pub const COLOR_WHITE: Color = Color {
    x: 1.,
    y: 1.,
    z: 1.,
};

/// 黑色
pub const COLOR_BLACK: Color = Color {
    x: 0.,
    y: 0.,
    z: 0.,
};

// static INTENSITY: LazyLock<Interval> = LazyLock::new(|| Interval::new(0., 0.999));

// TODO: 使用From替代
/// Translate a color into a tuple of bytes
pub fn translate_color(pixel_color: Color) -> Rgb {
    let (r, g, b) = pixel_color.into();

    // translate the [0, 1] component values to the byte range [0, 255]
    // Apply a linear to gamma transform for gamma 2
    Rgb {
        r: (255.999 * r) as u8,
        g: (255.999 * g) as u8,
        b: (255.999 * b) as u8,
    }
}

// Convert a linear component to a gamma component
// fn linear_to_gamma(linear_component: f64) -> f64 {
//     if linear_component > 0. {
//         linear_component.sqrt()
//     } else {
//         0.
//     }
// }
