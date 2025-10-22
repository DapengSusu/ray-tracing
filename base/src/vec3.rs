use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// Vector with three components.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vec3 {
    /// X component.
    pub x: f64,
    /// Y component.
    pub y: f64,
    /// Z component.
    pub z: f64,
}

impl Vec3 {
    /// Creates a new vector with the given components.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Returns an iterator over the components of the vector.
    pub fn iter(&self) -> Vec3Iter<'_> {
        Vec3Iter {
            vec3: self,
            index: 0,
        }
    }

    /// Returns the dot product of two vectors.
    pub fn dot(&self, other: &Self) -> f64 {
        dot(self, other)
    }

    /// Returns the cross product of two vectors.
    pub fn cross(&self, other: &Self) -> Vec3 {
        cross(self, other)
    }

    /// Returns the unit vector.
    pub fn to_unit(&self) -> Vec3 {
        unit_vec3(self)
    }

    /// Returns the squared length of the vector.
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the length of the vector.
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

/// Returns the dot product of two vectors.
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

/// Returns the cross product of two vectors.
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x,
    )
}

/// Returns the unit vector of a vector.
pub fn unit_vec3(v: &Vec3) -> Vec3 {
    *v / v.length()
}

/// Iterator over the components of a vector.
pub struct Vec3Iter<'a> {
    vec3: &'a Vec3,
    index: usize,
}

impl<'a> Iterator for Vec3Iter<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;

        match self.index {
            1 => Some(self.vec3.x),
            2 => Some(self.vec3.y),
            3 => Some(self.vec3.z),
            _ => None,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

// -v
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// v1 + v2
impl Add<Self> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// v1 - v2
impl Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// v1 * v2
impl Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

// v * scalar
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

// v1 / v2
impl Div<Self> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

// v / scalar
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        self * (1. / scalar)
    }
}

// v1 += v2
impl AddAssign<Self> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

// v1 -= v2
impl SubAssign<Self> for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

// v1 *= v2
impl MulAssign<Self> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        };
    }
}

// v *= scalar
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scalar: f64) {
        *self = Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        };
    }
}

// v1 /= v2
impl DivAssign<Self> for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        };
    }
}

// v /= scalar
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scalar: f64) {
        *self *= 1. / scalar;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_neg_should_work() {
        let v = Vec3::new(1., -2., 3.5);

        assert_eq!(-v, Vec3::new(-1., 2., -3.5));
    }

    #[test]
    fn vec3_add_should_work() {
        let mut v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(4., 5., 6.);

        assert_eq!(v1 + v2, Vec3::new(5., 7., 9.));

        v1 += v2;
        assert_eq!(v1, Vec3::new(5., 7., 9.));
    }

    #[test]
    fn vec3_sub_should_work() {
        let mut v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(4., 5., 6.);

        assert_eq!(v1 - v2, Vec3::new(-3., -3., -3.));

        v1 -= v2;
        assert_eq!(v1, Vec3::new(-3., -3., -3.));
    }

    #[test]
    fn vec3_mul_should_work() {
        let mut v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(4., 5., 6.);

        assert_eq!(v1 * v2, Vec3::new(4., 10., 18.));

        v1 *= v2;
        assert_eq!(v1, Vec3::new(4., 10., 18.));
    }

    #[test]
    fn vec3_div_should_work() {
        let mut v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(4., 5., 6.);

        assert_eq!(v1 / v2, Vec3::new(0.25, 0.4, 0.5));

        v1 /= v2;
        assert_eq!(v1, Vec3::new(0.25, 0.4, 0.5));
    }

    #[test]
    fn vec3_mul_scalar_should_work() {
        let mut v = Vec3::new(1., 2., 3.);

        assert_eq!(v * 2., Vec3::new(2., 4., 6.));

        v *= 2.;
        assert_eq!(v, Vec3::new(2., 4., 6.));
    }

    #[test]
    fn vec3_div_scalar_should_work() {
        let mut v = Vec3::new(1., 2., 3.);

        assert_eq!(v / 2., Vec3::new(0.5, 1., 1.5));

        v /= 2.;
        assert_eq!(v, Vec3::new(0.5, 1., 1.5));
    }

    #[test]
    fn vec3_dot_should_work() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(4., 5., 6.);

        assert_eq!(v1.dot(&v2), 32.);
    }

    #[test]
    fn vec3_cross_should_work() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(4., 5., 6.);

        assert_eq!(v1.cross(&v2), Vec3::new(-3., 6., -3.));
    }

    #[test]
    fn vec3_to_unit_should_work() {
        let v = Vec3::new(1., 2., 3.);

        assert_eq!(v.to_unit().length(), 1.);
    }
}
