use std::sync::Arc;

use crate::prelude::*;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Option<Arc<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Option<Arc<dyn Material>>) -> Self {
        Sphere {
            center,
            radius: radius.max(0.),
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;

        let hit_record = HitRecord::builder()
            .set_t(t)
            .set_p(p)
            .set_face_normal(ray, outward_normal)
            .set_material(self.material.clone());

        Some(hit_record)
    }
}
