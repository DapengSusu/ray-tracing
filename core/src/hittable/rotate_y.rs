use std::sync::Arc;

use crate::{
    Point3, Vec3,
    aabb::AABB,
    common::Degrees,
    ray::Ray,
    texture::{HitRecord, Hittable, HittableObject, Interval},
};

/// For y-rotation
#[derive(Debug, Clone)]
pub struct RotateY {
    object: Arc<HittableObject>,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: AABB,
}

impl RotateY {
    pub fn new(object: Arc<HittableObject>, angle: Degrees) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::with_isotropic(f64::INFINITY);
        let mut max = Point3::with_isotropic(f64::NEG_INFINITY);

        (0..2).for_each(|i| {
            (0..2).for_each(|j| {
                (0..2).for_each(|k| {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                });
            });
        });

        Self {
            object,
            sin_theta,
            cos_theta,
            bounding_box: AABB::with_points(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Transform the ray from world space to object space.

        let origin = Point3::new(
            (self.cos_theta * ray.origin.x) - (self.sin_theta * ray.origin.z),
            ray.origin.y,
            (self.sin_theta * ray.origin.x) + (self.cos_theta * ray.origin.z),
        );

        let direction = Vec3::new(
            (self.cos_theta * ray.direction.x) - (self.sin_theta * ray.direction.z),
            ray.direction.y,
            (self.sin_theta * ray.direction.x) + (self.cos_theta * ray.direction.z),
        );

        let rotated_r = Ray::new_with_time(origin, direction, ray.time);

        // Determine whether an intersection exists in object space (and if so, where).
        if let Some(mut hit_record) = self.object.hit(&rotated_r, ray_t) {
            // Transform the intersection from object space back to world space.
            hit_record.p = Point3::new(
                (self.cos_theta * hit_record.p.x) + (self.sin_theta * hit_record.p.z),
                hit_record.p.y,
                (-self.sin_theta * hit_record.p.x) + (self.cos_theta * hit_record.p.z),
            );

            hit_record.normal = Vec3::new(
                (self.cos_theta * hit_record.normal.x) + (self.sin_theta * hit_record.normal.z),
                hit_record.normal.y,
                (-self.sin_theta * hit_record.normal.x) + (self.cos_theta * hit_record.normal.z),
            );

            Some(hit_record)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}
