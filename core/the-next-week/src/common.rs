use std::{ops::Deref, sync::LazyLock};

use rand::distr::{Distribution, Uniform, uniform::SampleUniform};

static RANDOM_RANGE: LazyLock<Uniform<f64>> = LazyLock::new(|| Uniform::new(0., 1.).unwrap());

/// 角度
#[derive(Debug, Default)]
pub struct Degrees(pub(crate) f64);

/// 弧度
#[derive(Debug, Default)]
pub struct Radians(pub(crate) f64);

impl Deref for Degrees {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Radians {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Generate a random floating-point number between 0 and 1.
///
/// # Examples
///
/// ```rust
/// # use the_next_week_core::common;
/// assert!((0. ..1.).contains(&common::random()));
/// ```
///
/// # Note
///
/// [0, 1)
pub fn random() -> f64 {
    RANDOM_RANGE.sample(&mut rand::rng())
}

/// Generate a random value between `min` and `max`.
///
/// # Examples
///
/// ```rust
/// # use the_next_week_core::common;
/// // floating-point number
/// assert!((5.2..12.5).contains(&common::random_range(5.2, 12.5)));
/// // integer
/// assert!((5..12).contains(&common::random_range(5, 12)));
/// // character
/// assert!(('a'..='z').contains(&common::random_range('a', 'z')));
/// ```
///
/// # Note
///
/// [min, max)
pub fn random_range<T: SampleUniform>(min: T, max: T) -> T {
    Uniform::new(min, max).unwrap().sample(&mut rand::rng())
}

/// Check if two f64 values are relatively equal within a given epsilon.
///
/// # Note
///
/// if `epsilon` is `None`, the default epsilon is 1e-8.
/// but if `epsilon` is `Some(...)`, it will be less than 1e-12.
pub fn relative_eq(a: f64, b: f64, epsilon: Option<f64>) -> bool {
    let max_val = a.abs().max(b.abs()).max(f64::MIN_POSITIVE);

    (a - b).abs() < epsilon.unwrap_or(1e-8).max(1e-12) * max_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_should_between_0_and_1() {
        assert!((0. ..1.).contains(&random()));
    }

    #[test]
    fn random_range_should_work() {
        assert!((5.2..12.5).contains(&random_range(5.2, 12.5)));
    }

    #[test]
    fn relative_eq_should_work() {
        assert!(relative_eq(1., 1.000000001, None));
        assert!(relative_eq(1., 1.00000001, None));
        assert!(!relative_eq(1., 1.00000002, None));
    }
}
