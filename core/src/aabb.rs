use std::ops::{Add, AddAssign};
use std::{iter::Sum, ops::Index};

use crate::interval;
use crate::prelude::*;

/// Empty axis-aligned bounding box.
pub const EMPTY: AABB = AABB {
    x: interval::EMPTY,
    y: interval::EMPTY,
    z: interval::EMPTY,
};

/// Universe axis-aligned bounding box.
pub const UNIVERSE: AABB = AABB {
    x: interval::UNIVERSE,
    y: interval::UNIVERSE,
    z: interval::UNIVERSE,
};

/// Axis-aligned bounding boxes.（轴对齐边界框）
/// The default AABB is empty, since intervals are empty by default.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    /// Create a new AABB from the given intervals.
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }.pad_to_minimums()
    }

    /// Construct an axis-aligned bounding box with two points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ray_tracing_core::aabb::AABB;
    /// # use ray_tracing_core::interval::Interval;
    /// # use ray_tracing_core::Point3;
    /// let a = Point3::new(1., 2., 3.);
    /// let b = Point3::new(1.5, 1.5, 3.5);
    /// let expected = AABB::new(
    ///     Interval::new(1., 1.5),
    ///     Interval::new(1.5, 2.),
    ///     Interval::new(3., 3.5),
    /// );
    /// assert_eq!(AABB::with_points(a, b), expected);
    /// ```
    pub fn with_points(a: Point3, b: Point3) -> Self {
        // Treat the two points a and b as extrema for the bounding box, so we don't require a
        // particular minimum/maximum coordinate order.
        Self {
            x: Interval::new(a.x.min(b.x), a.x.max(b.x)),
            y: Interval::new(a.y.min(b.y), a.y.max(b.y)),
            z: Interval::new(a.z.min(b.z), a.z.max(b.z)),
        }
        .pad_to_minimums()
    }

    /// Construct an axis-aligned bounding box from two input boxes.
    pub fn from_boxes(box0: &AABB, box1: &AABB) -> Self {
        Self {
            x: Interval::with_enclosing(&box0.x, &box1.x),
            y: Interval::with_enclosing(&box0.y, &box1.y),
            z: Interval::with_enclosing(&box0.z, &box1.z),
        }
    }

    pub fn append(mut self, bbox: AABB) -> Self {
        self += bbox;

        self
    }

    pub fn replace(&mut self, bbox: AABB) {
        *self = bbox;
    }

    /// Returns the index of the longest axis of the bounding box.
    pub fn longest_axis(&self) -> u8 {
        let x_size = self.x.size();
        let y_size = self.y.size();
        let z_size = self.z.size();

        if x_size > y_size {
            if x_size > z_size { 0 } else { 2 }
        } else if y_size > z_size {
            1
        } else {
            2
        }
    }

    pub fn hit(&self, ray: &Ray, mut ray_t: Interval) -> bool {
        let ray_origin = &ray.origin;
        let ray_direc = &ray.direction;

        for axis in 0..3 {
            let ax = &self[axis];
            let adinv = ray_direc[axis].recip();

            let t0 = (ax.min - ray_origin[axis]) * adinv;
            let t1 = (ax.max - ray_origin[axis]) * adinv;

            ray_t.min = ray_t.min.max(t0.min(t1));
            ray_t.max = ray_t.max.min(t0.max(t1));

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }

    // Adjust the AABB so that no side is narrower than some delta, padding if necessary.
    fn pad_to_minimums(mut self) -> Self {
        const DELTA: f64 = 0.0001;

        if self.x.size() < DELTA {
            self.x.expand(DELTA);
        }
        if self.y.size() < DELTA {
            self.y.expand(DELTA);
        }
        if self.z.size() < DELTA {
            self.z.expand(DELTA);
        }

        self
    }
}

impl Index<u8> for AABB {
    type Output = Interval;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(
                "Index out of bounds, only 0, 1, 2 are valid indices, but got {}",
                index
            ),
        }
    }
}

impl Sum for AABB {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::default(), |acc, item| Self::from_boxes(&acc, &item))
    }
}

impl Add for AABB {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::from_boxes(&self, &other)
    }
}

impl AddAssign for AABB {
    fn add_assign(&mut self, other: Self) {
        *self = Self::from_boxes(self, &other);
    }
}

impl Add<Vec3> for AABB {
    type Output = AABB;

    fn add(self, rhs: Vec3) -> Self::Output {
        AABB {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Vec3> for &AABB {
    type Output = AABB;

    fn add(self, rhs: Vec3) -> Self::Output {
        *self + rhs
    }
}

impl Add<AABB> for Vec3 {
    type Output = AABB;

    fn add(self, rhs: AABB) -> Self::Output {
        rhs + self
    }
}

impl Add<&AABB> for Vec3 {
    type Output = AABB;

    fn add(self, rhs: &AABB) -> Self::Output {
        self + *rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aabb_index_should_work() {
        let aabb = AABB::new(
            Interval::new(0.0, 1.0),
            Interval::new(2.0, 3.0),
            Interval::new(4.0, 5.0),
        );

        assert_eq!(aabb[0], Interval::new(0.0, 1.0));
        assert_eq!(aabb[1], Interval::new(2.0, 3.0));
        assert_eq!(aabb[2], Interval::new(4.0, 5.0));
    }

    #[test]
    fn aabb_sum_should_work() {
        let aabb1 = AABB::new(
            Interval::new(0.0, 1.0),
            Interval::new(2.0, 3.0),
            Interval::new(4.0, 5.0),
        );
        let aabb2 = AABB::new(
            Interval::new(6.0, 7.0),
            Interval::new(8.0, 9.0),
            Interval::new(10.0, 11.0),
        );

        let sum: AABB = [aabb1, aabb2].into_iter().sum();

        assert_eq!(sum.x, Interval::new(0.0, 7.0));
        assert_eq!(sum.y, Interval::new(2.0, 9.0));
        assert_eq!(sum.z, Interval::new(4.0, 11.0));
    }
}
