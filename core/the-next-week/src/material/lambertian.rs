use crate::prelude::*;

/// Lambertian material
#[derive(Default)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    /// Create a new Lambertian material with the given albedo color.
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new_with_time(hit.p, scatter_direction, ray_in.time);

        Some((self.albedo, scattered))
    }
}
