use std::sync::Arc;

use crate::{
    Vec3,
    aabb::AABB,
    ray::Ray,
    texture::{HitRecord, Hittable, HittableObject, Interval},
};

/// We need to move the intersection point forward the offset amount so that
/// the intersection is actually in the path of the incident ray.
#[derive(Debug, Clone)]
pub struct Translate {
    object: Arc<HittableObject>,
    offset: Vec3,
    bounding_box: AABB,
}

impl Translate {
    pub fn new(object: Arc<HittableObject>, offset: Vec3) -> Self {
        Self {
            object: object.clone(),
            offset,
            bounding_box: object.bounding_box() + offset,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Move the ray backwards by the offset
        let offset_r = Ray::new_with_time(ray.origin - self.offset, ray.direction, ray.time);

        // Determine whether an intersection exists along the offset ray (and if so, where)
        if let Some(mut hit_record) = self.object.hit(&offset_r, ray_t) {
            // Move the intersection point forwards by the offset
            hit_record.p += self.offset;

            Some(hit_record)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}
