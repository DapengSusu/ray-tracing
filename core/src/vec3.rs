use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// 带有三个分量的向量
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vec3 {
    /// X 分量.
    pub x: f64,
    /// Y 分量.
    pub y: f64,
    /// Z 分量.
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    pub const ONE: Vec3 = Vec3 {
        x: 1.,
        y: 1.,
        z: 1.,
    };

    pub const UP: Vec3 = Vec3 {
        x: 0.,
        y: 1.,
        z: 0.,
    };
}

impl Vec3 {
    /// Creates a new vector with the given components.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Creates a new vector with all components set to the same value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ray_tracing_core::Vec3;
    /// let v = Vec3::with_isotropic(2.);
    /// assert_eq!(v, Vec3::new(2., 2., 2.));
    /// ```
    pub fn with_isotropic(value: f64) -> Self {
        Self::new(value, value, value)
    }

    pub fn set_x(mut self, x: f64) -> Self {
        self.x = x;
        self
    }

    pub fn set_y(mut self, y: f64) -> Self {
        self.y = y;
        self
    }

    pub fn set_z(mut self, z: f64) -> Self {
        self.z = z;
        self
    }
}

impl Vec3 {
    /// Return true if the vector is close to zero in all dimensions.
    pub fn near_zero(&self) -> bool {
        self.near_zero_by(Some(1e-8))
    }

    /// Return true if the vector is close to zero in all dimensions.
    pub fn near_zero_by(&self, epsilon: Option<f64>) -> bool {
        let epsilon = epsilon.unwrap_or(1e-8).max(1e-12);

        self.x.abs() < epsilon && self.y.abs() < epsilon && self.z.abs() < epsilon
    }

    /// Returns an iterator over the components of the vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ray_tracing_core::Vec3;
    /// let v = Vec3::new(1., 2., 3.);
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
    /// # use ray_tracing_core::Vec3;
    /// let v = Vec3::new(1., 2., 3.);
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
    /// # use ray_tracing_core::Vec3;
    /// let v1 = Vec3::new(1., 2., 3.);
    /// let v2 = Vec3::new(4., 5., 6.);
    /// let v3 = v1.cross(&v2);
    /// assert_eq!(v3, Vec3::new(-3., 6., -3.));
    ///
    /// let v4 = Vec3::new(
    ///     v1.y * v2.z - v1.z * v2.y,
    ///     v1.z * v2.x - v1.x * v2.z,
    ///     v1.x * v2.y - v1.y * v2.x
    /// );
    /// assert_eq!(v3, v4);
    /// ```
    pub fn cross(&self, other: &Self) -> Self {
        cross(self, other)
    }

    /// Returns the unit vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ray_tracing_core::Vec3;
    /// let v = Vec3::new(1., 2., 3.);
    /// let u = v.to_unit();
    /// assert_eq!(u.length(), 1.);
    /// ```
    ///
    /// # Note
    ///
    /// Need to calculate the length of the vector by using `sqrt()`
    pub fn to_unit(&self) -> Self {
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

/// Calculates the reflection.
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2. * dot(v, n) * *n
}

/// Calculates the refraction.
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-*uv).dot(n).min(1.);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -((1. - r_out_perp.length_squared()).abs().sqrt()) * *n;

    r_out_perp + r_out_parallel
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
            _ => panic!("Index out of bounds, got {index}"),
        }
    }
}

impl IndexMut<u8> for Vec3 {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds, got {index}"),
        }
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::ZERO, |acc, v| acc + v)
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
impl Add for Vec3 {
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
impl Sub for Vec3 {
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
impl Mul for Vec3 {
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

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
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

// v1 / v2
impl Div for Vec3 {
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

    fn div(self, rhs: f64) -> Self::Output {
        self * (1. / rhs)
    }
}

// v1 += v2
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

// v1 -= v2
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

// v1 *= v2
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

// v *= scalar
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

// v1 /= v2
impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

// v /= scalar
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1. / rhs;
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

    #[test]
    fn vec3_index_should_work() {
        let v = Vec3::new(1., 2., 3.);

        assert_eq!(v[0], 1.);
        assert_eq!(v[1], 2.);
        assert_eq!(v[2], 3.);
    }

    #[test]
    fn vec3_index_mut_should_work() {
        let mut v = Vec3::new(1., 2., 3.);

        v[0] = 0.5;
        v[1] = 2.5;

        assert_eq!(v, Vec3::new(0.5, 2.5, 3.));
    }
}
