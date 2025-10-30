/// Manage real-valued intervals with a minimum and a maximum.
#[derive(Debug, Clone, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

// Default interval is empty
impl Default for Interval {
    fn default() -> Self {
        INTERVAL_EMPTY
    }
}

/// Empty interval.
pub const INTERVAL_EMPTY: Interval = Interval {
    min: f64::INFINITY,
    max: f64::NEG_INFINITY,
};

/// Universe interval.
pub const INTERVAL_UNIVERSE: Interval = Interval {
    min: f64::NEG_INFINITY,
    max: f64::INFINITY,
};

impl Interval {
    /// Create a new interval from a minimum and a maximum.
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Check if the interval contains a value.
    ///
    /// # Tip
    ///
    /// min <= x && x <= max
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// Check if the interval surrounds a value.
    ///
    /// # Tip
    ///
    /// min < x && x < max
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    /// Calculate the size of the interval.
    ///
    /// # Tip
    ///
    /// max - min
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// Clamp a value to the interval.
    ///
    /// # Tip
    ///
    /// x.max(min).min(max)
    /// min <= x && x <= max
    ///
    /// # Panics
    ///
    /// Panics if `min > max`, `min` is NaN, or `max` is NaN.
    pub fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_default_is_empty() {
        let interval = Interval::default();

        assert_eq!(interval, INTERVAL_EMPTY);
    }

    #[test]
    fn interval_contains_should_work() {
        let interval = Interval::new(1., 2.);

        assert!(interval.contains(1.));
        assert!(interval.contains(2.));
        assert!(interval.contains(1.5));
        assert!(!interval.contains(0.5));
        assert!(!interval.contains(2.5));
    }

    #[test]
    fn interval_surrounds_should_work() {
        let interval = Interval::new(1., 2.);

        assert!(!interval.surrounds(1.));
        assert!(!interval.surrounds(2.));
        assert!(interval.surrounds(1.5));
        assert!(!interval.surrounds(0.5));
        assert!(!interval.surrounds(2.5));
    }

    #[test]
    fn interval_size_should_work() {
        let interval = Interval::new(1.2, 2.5);

        assert_eq!(interval.size(), 1.3);
    }

    #[test]
    fn interval_clamp_should_work() {
        let interval = Interval::new(1.2, 2.5);

        assert_eq!(interval.clamp(0.5), 1.2);
        assert_eq!(interval.clamp(1.5), 1.5);
        assert_eq!(interval.clamp(2.5), 2.5);
        assert_eq!(interval.clamp(3.0), 2.5);
    }
}
