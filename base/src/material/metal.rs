use crate::{prelude::*, vec3};

/// Metal material
#[derive(Default)]
pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    /// Create a new metal material with the given albedo color.
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = vec3::reflect(&ray_in.direction, &hit.normal);

        Some((self.albedo, Ray::new(hit.p, reflected)))
    }
}
