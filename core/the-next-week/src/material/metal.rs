use crate::{prelude::*, vec3};

/// Metal material
#[derive(Default)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    /// Create a new metal material with the given albedo color and fuzziness.
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0., 1.),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = vec3::reflect(&ray_in.direction, &hit.normal);
        let reflected = reflected.to_unit() + self.fuzz * Vec3::random_unit_vector();
        let scattered = Ray::new_with_time(hit.p, reflected, ray_in.time);

        if scattered.direction.dot(&hit.normal) > 0. {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
