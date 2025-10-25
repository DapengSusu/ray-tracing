use crate::prelude::*;

/// 电介质
pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflect(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1. - refraction_index) / (1. + refraction_index);
        let r0 = r0 * r0;

        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let ri = if hit.front_face {
            1. / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray_in.direction.to_unit();
        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.;

        let direction = if cannot_refract || Self::reflect(cos_theta, ri) > common::random() {
            // reflection
            vec3::reflect(&unit_direction, &hit.normal)
        } else {
            // refraction
            vec3::refract(&unit_direction, &hit.normal, ri)
        };
        let scattered = Ray::new_with_time(hit.p, direction, ray_in.time);

        Some((Color::one(), scattered))
    }
}
