use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign},
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
    /// Creates a new vector with all components set to zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ray_tracing_base::Vec3;
    /// let v = Vec3::new();
    /// assert_eq!(v, Vec3::zero());
    /// assert_eq!(v, Vec3::from_xyz(0., 0., 0.));
    /// ```
    pub fn new() -> Self {
        Self::zero()
    }

    /// Creates a new vector with the given components.
    pub fn from_xyz(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Creates a new vector with all components set to zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ray_tracing_base::Vec3;
    /// let v = Vec3::zero();
    /// assert_eq!(v, Vec3::from_xyz(0., 0., 0.));
    /// ```
    pub fn zero() -> Self {
        Self::from_xyz(0., 0., 0.)
    }

    /// Creates a new vector with all components set to one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ray_tracing_base::Vec3;
    /// let v = Vec3::one();
    /// assert_eq!(v, Vec3::from_xyz(1., 1., 1.));
    /// ```
    pub fn one() -> Self {
        Self::from_xyz(1., 1., 1.)
    }

    /// Creates a new vector with all components set to the given value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ray_tracing_base::Vec3;
    /// let v = Vec3::from_scalar(2.);
    /// assert_eq!(v, Vec3::from_xyz(2., 2., 2.));
    /// ```
    pub fn from_scalar(value: f64) -> Self {
        Self::from_xyz(value, value, value)
    }

    /// Creates a new vector with the given x component and y and z components set to zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ray_tracing_base::Vec3;
    /// let v = Vec3::from_x(2.);
    /// assert_eq!(v, Vec3::from_xyz(2., 0., 0.));
    /// ```
    pub fn from_x(x: f64) -> Self {
        Self::from_xyz(x, 0., 0.)
    }

    /// Creates a new vector with the given y component and x and z components set to zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ray_tracing_base::Vec3;
    /// let v = Vec3::from_y(2.);
    /// assert_eq!(v, Vec3::from_xyz(0., 2., 0.));
    /// ```
    pub fn from_y(y: f64) -> Self {
        Self::from_xyz(0., y, 0.)
    }

    /// Creates a new vector with the given z component and x and y components set to zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ray_tracing_base::Vec3;
    /// let v = Vec3::from_z(2.);
    /// assert_eq!(v, Vec3::from_xyz(0., 0., 2.));
    /// ```
    pub fn from_z(z: f64) -> Self {
        Self::from_xyz(0., 0., z)
    }

    /// Creates a new vector with the given x and y components and z component set to zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ray_tracing_base::Vec3;
    /// let v = Vec3::from_xy(2., 3.);
    /// assert_eq!(v, Vec3::from_xyz(2., 3., 0.));
    /// ```
    pub fn from_xy(x: f64, y: f64) -> Self {
        Self::from_xyz(x, y, 0.)
    }

    /// Creates a new vector with the given x and z components and y component set to zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ray_tracing_base::Vec3;
    /// let v = Vec3::from_xz(2., 3.);
    /// assert_eq!(v, Vec3::from_xyz(2., 0., 3.));
    /// ```
    pub fn from_xz(x: f64, z: f64) -> Self {
        Self::from_xyz(x, 0., z)
    }

    /// Creates a new vector with the given y and z components and x component set to zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ray_tracing_base::Vec3;
    /// let v = Vec3::from_yz(2., 3.);
    /// assert_eq!(v, Vec3::from_xyz(0., 2., 3.));
    /// ```
    pub fn from_yz(y: f64, z: f64) -> Self {
        Self::from_xyz(0., y, z)
    }

    /// Returns an iterator over the components of the vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ray_tracing_base::Vec3;
    /// let v = Vec3::from_xyz(1., 2., 3.);
    /// let mut iter = v.iter();
    /// assert_eq!(iter.next(), Some(1.));
    /// assert_eq!(iter.next(), Some(2.));
    /// assert_eq!(iter.next(), Some(3.));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Vec3Iter<'_> {
        Vec3Iter {
            vec3: self,
            index: 0,
        }
    }

    /// Returns the dot product of two vectors. (Be like: v1 * v2)
    ///
    /// Tip: v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    pub fn dot(&self, other: &Self) -> f64 {
        dot(self, other)
    }

    /// Returns the dot product of a vector with itself.
    ///
    /// Tip: v.dot(&v)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ray_tracing_base::Vec3;
    /// let v = Vec3::from_xyz(1., 2., 3.);
    /// assert_eq!(v.dot_self(), v.dot(&v));
    /// ```
    pub fn dot_self(&self) -> f64 {
        dot(self, self)
    }

    /// Returns the cross product of two vectors. (Be like: v1 x v2)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use ray_tracing_base::Vec3;
    /// let v1 = Vec3::from_xyz(1., 2., 3.);
    /// let v2 = Vec3::from_xyz(4., 5., 6.);
    /// let v3 = v1.cross(&v2);
    /// assert_eq!(v3, Vec3::from_xyz(-3., 6., -3.));
    ///
    /// let v4 = Vec3::from_xyz(
    ///     v1.y * v2.z - v1.z * v2.y,
    ///     v1.z * v2.x - v1.x * v2.z,
    ///     v1.x * v2.y - v1.y * v2.x
    /// );
    /// assert_eq!(v3, v4);
    /// ```
    pub fn cross(&self, other: &Self) -> Vec3 {
        cross(self, other)
    }

    /// Returns the unit vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ray_tracing_base::Vec3;
    /// let v = Vec3::from_xyz(1., 2., 3.);
    /// let u = v.to_unit();
    /// assert_eq!(u.length(), 1.);
    /// ```
    pub fn to_unit(&self) -> Vec3 {
        unit_vec3(self)
    }

    /// Returns the squared length of the vector.
    ///
    /// Tip: v.x * v.x + v.y * v.y + v.z * v.z
    pub fn length_squared(&self) -> f64 {
        self.dot_self()
    }

    /// Returns the length of the vector. (magnitude)
    ///
    /// Tip: v.length_squared().sqrt()
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
    Vec3::from_xyz(
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

impl Index<u8> for Vec3 {
    type Output = f64;

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

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

// Converts Vec3 to a tuple, like (x, y, z).
impl From<Vec3> for (f64, f64, f64) {
    fn from(vec3: Vec3) -> Self {
        (vec3.x, vec3.y, vec3.z)
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

// scalar * v
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

// v * scalar
impl Mul<u32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: u32) -> Self::Output {
        self * scalar as f64
    }
}

// scalar * v
impl Mul<Vec3> for u32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self as f64
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

// v / scalar
impl Div<u32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: u32) -> Self::Output {
        self / scalar as f64
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

// v *= scalar
impl MulAssign<u32> for Vec3 {
    fn mul_assign(&mut self, scalar: u32) {
        *self *= scalar as f64;
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

// v /= scalar
impl DivAssign<u32> for Vec3 {
    fn div_assign(&mut self, scalar: u32) {
        *self /= scalar as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_neg_should_work() {
        let v = Vec3::from_xyz(1., -2., 3.5);

        assert_eq!(-v, Vec3::from_xyz(-1., 2., -3.5));
    }

    #[test]
    fn vec3_add_should_work() {
        let mut v1 = Vec3::from_xyz(1., 2., 3.);
        let v2 = Vec3::from_xyz(4., 5., 6.);

        assert_eq!(v1 + v2, Vec3::from_xyz(5., 7., 9.));

        v1 += v2;
        assert_eq!(v1, Vec3::from_xyz(5., 7., 9.));
    }

    #[test]
    fn vec3_sub_should_work() {
        let mut v1 = Vec3::from_xyz(1., 2., 3.);
        let v2 = Vec3::from_xyz(4., 5., 6.);

        assert_eq!(v1 - v2, Vec3::from_xyz(-3., -3., -3.));

        v1 -= v2;
        assert_eq!(v1, Vec3::from_xyz(-3., -3., -3.));
    }

    #[test]
    fn vec3_mul_should_work() {
        let mut v1 = Vec3::from_xyz(1., 2., 3.);
        let v2 = Vec3::from_xyz(4., 5., 6.);

        assert_eq!(v1 * v2, Vec3::from_xyz(4., 10., 18.));

        v1 *= v2;
        assert_eq!(v1, Vec3::from_xyz(4., 10., 18.));
    }

    #[test]
    fn vec3_div_should_work() {
        let mut v1 = Vec3::from_xyz(1., 2., 3.);
        let v2 = Vec3::from_xyz(4., 5., 6.);

        assert_eq!(v1 / v2, Vec3::from_xyz(0.25, 0.4, 0.5));

        v1 /= v2;
        assert_eq!(v1, Vec3::from_xyz(0.25, 0.4, 0.5));
    }

    #[test]
    fn vec3_mul_scalar_should_work() {
        let mut v = Vec3::from_xyz(1., 2., 3.);

        assert_eq!(v * 2., Vec3::from_xyz(2., 4., 6.));

        v *= 2.;
        assert_eq!(v, Vec3::from_xyz(2., 4., 6.));
    }

    #[test]
    fn vec3_div_scalar_should_work() {
        let mut v = Vec3::from_xyz(1., 2., 3.);

        assert_eq!(v / 2., Vec3::from_xyz(0.5, 1., 1.5));

        v /= 2.;
        assert_eq!(v, Vec3::from_xyz(0.5, 1., 1.5));
    }

    #[test]
    fn vec3_dot_should_work() {
        let v1 = Vec3::from_xyz(1., 2., 3.);
        let v2 = Vec3::from_xyz(4., 5., 6.);

        assert_eq!(v1.dot(&v2), 32.);
    }

    #[test]
    fn vec3_cross_should_work() {
        let v1 = Vec3::from_xyz(1., 2., 3.);
        let v2 = Vec3::from_xyz(4., 5., 6.);

        assert_eq!(v1.cross(&v2), Vec3::from_xyz(-3., 6., -3.));
    }

    #[test]
    fn vec3_to_unit_should_work() {
        let v = Vec3::from_xyz(1., 2., 3.);

        assert_eq!(v.to_unit().length(), 1.);
    }

    #[test]
    fn vec3_index_should_work() {
        let v = Vec3::from_xyz(1., 2., 3.);

        assert_eq!(v[0], 1.);
        assert_eq!(v[1], 2.);
        assert_eq!(v[2], 3.);
    }
}
