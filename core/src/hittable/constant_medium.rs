use std::sync::Arc;

use crate::{
    Color, Vec3,
    aabb::AABB,
    common::{self, UvCoord},
    interval,
    ray::Ray,
    texture::{
        HitRecord, Hittable, HittableObject, Interval, Isotropic, MaterialType, TextureType,
    },
};

/// 恒密度介质
#[derive(Debug, Clone)]
pub struct ConstantMedium {
    boundary: Arc<HittableObject>,
    phase_function: Arc<MaterialType>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn with_texture(
        boundary: Arc<HittableObject>,
        density: f64,
        tex: Arc<TextureType>,
    ) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(MaterialType::Isotropic(Isotropic::new(tex))),
            neg_inv_density: -1. / density,
        }
    }

    pub fn with_color(boundary: Arc<HittableObject>, density: f64, albedo: Color) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(MaterialType::Isotropic(Isotropic::with_color(albedo))),
            neg_inv_density: -1. / density,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if let Some(mut hit1) = self.boundary.hit(ray, interval::UNIVERSE)
            && let Some(mut hit2) = self
                .boundary
                .hit(ray, Interval::new(hit1.t + 0.0001, f64::INFINITY))
        {
            hit1.t = hit1.t.max(ray_t.min);
            hit2.t = hit2.t.min(ray_t.max);

            if hit1.t >= hit2.t {
                return None;
            }

            hit1.t = hit1.t.max(0.);

            let ray_length = ray.direction.length();
            let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
            let hit_distance = self.neg_inv_density * common::random().ln();

            if hit_distance > distance_inside_boundary {
                return None;
            }

            let t = hit1.t + hit_distance / ray_length;
            let p = ray.at(t);

            let normal = Vec3::with_x(1.); // arbitrary
            let front_face = true; // also arbitrary

            return Some(HitRecord {
                t,
                uv: UvCoord::default(),
                p,
                normal,
                front_face,
                material: Some(self.phase_function.clone()),
            });
        }

        None
    }

    fn bounding_box(&self) -> &AABB {
        self.boundary.bounding_box()
    }
}
