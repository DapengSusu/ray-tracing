use std::sync::LazyLock;

use rand::distr::{Distribution, Uniform};

static RANDOM_RANGE: LazyLock<Uniform<f64>> = LazyLock::new(|| Uniform::new(0., 1.).unwrap());

#[derive(Debug, Default)]
pub struct Degrees(f64);

#[derive(Debug, Default)]
pub struct Radians(f64);

impl From<Degrees> for Radians {
    fn from(degrees: Degrees) -> Self {
        Radians(degrees.0.to_radians())
    }
}

impl From<Radians> for Degrees {
    fn from(radians: Radians) -> Self {
        Degrees(radians.0.to_degrees())
    }
}

/// Generate a random floating-point number between 0 and 1.
///
/// # Examples
///
/// ```rust
/// # use ray_tracing_base::common;
/// assert!((0. ..1.).contains(&common::random()));
/// ```
///
/// # Note
///
/// [0, 1)
pub fn random() -> f64 {
    RANDOM_RANGE.sample(&mut rand::rng())
}

/// Generate a random floating-point number between `min` and `max`.
///
/// # Examples
///
/// ```rust
/// # use ray_tracing_base::common;
/// assert!((5.2..12.5).contains(&common::random_range(5.2, 12.5)));
/// ```
///
/// # Note
///
/// [min, max)
pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
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
}
