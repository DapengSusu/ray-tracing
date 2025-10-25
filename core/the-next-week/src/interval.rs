/// Manage real-valued intervals with a minimum and a maximum.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

/// Empty interval.
pub const EMPTY: Interval = Interval {
    min: f64::INFINITY,
    max: f64::NEG_INFINITY,
};

/// Universe interval.
pub const UNIVERSE: Interval = Interval {
    min: f64::NEG_INFINITY,
    max: f64::INFINITY,
};

impl Interval {
    /// Create a new interval from a minimum and a maximum.
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Create the interval tightly enclosing the two input intervals.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use the_next_week_core::interval::Interval;
    /// let interval1 = Interval::new(0.0, 1.0);
    /// let interval2 = Interval::new(2.0, 3.0);
    /// let expected = Interval::new(0.0, 3.0);
    ///
    /// assert_eq!(Interval::with_enclosing(&interval1, &interval2), expected);
    /// ```
    pub fn with_enclosing(a: &Self, b: &Self) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
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

    /// Extend the interval by a given delta, padding is half of the delta.
    /// `min` will subtract padding, `max` will add padding.
    pub fn extend(&self, delta: f64) -> Self {
        let padding = delta / 2.;

        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}

// Default interval is empty
impl Default for Interval {
    fn default() -> Self {
        EMPTY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_default_is_empty() {
        let interval = Interval::default();

        assert_eq!(interval, EMPTY);
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

    #[test]
    fn interval_extend_should_work() {
        let interval = Interval::new(1.2, 2.5);

        assert_eq!(interval.extend(1.), Interval::new(0.7, 3.0));
    }

    #[test]
    fn interval_with_enclosing_should_work() {
        let interval1 = Interval::new(1.2, 2.5);
        let interval2 = Interval::new(0.5, 3.0);

        assert_eq!(
            Interval::with_enclosing(&interval1, &interval2),
            Interval::new(0.5, 3.0)
        );
    }
}
