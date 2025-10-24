use std::sync::Arc;

use crate::prelude::*;

pub struct Sphere {
    center: Ray,
    radius: f64,
    material: Option<Arc<dyn Material>>,
    bounding_box: AABB,
}

impl Sphere {
    /// Create stationary sphere
    pub fn new(static_center: Point3, radius: f64, material: Option<Arc<dyn Material>>) -> Self {
        let rvec = Vec3::with_isotropic(radius);

        Sphere {
            center: Ray::new(static_center, Vec3::zero()),
            radius: radius.max(0.),
            material,
            bounding_box: AABB::with_points(static_center - rvec, static_center + rvec),
        }
    }

    /// Create moving sphere
    ///
    /// Replacing the 3D center point with a 3D ray that describes the original position
    /// at time=0 and the displacement to the end position at time=1.
    pub fn new_moving(
        center_original: Point3,
        center_end: Point3,
        radius: f64,
        material: Option<Arc<dyn Material>>,
    ) -> Self {
        let center = Ray::new(center_original, center_end - center_original);
        let rvec = Vec3::with_isotropic(radius);
        let box0 = AABB::with_points(center.at(0.) - rvec, center.at(0.) + rvec);
        let box1 = AABB::with_points(center.at(1.) - rvec, center.at(1.) + rvec);

        Sphere {
            center,
            radius: radius.max(0.),
            material,
            bounding_box: AABB::from_boxes(&box0, &box1),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time);
        let oc = current_center - ray.origin;
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
        let outward_normal = (p - current_center) / self.radius;

        let hit_record = HitRecord::builder()
            .set_t(t)
            .set_p(p)
            .set_face_normal(ray, outward_normal)
            .set_material(self.material.clone());

        Some(hit_record)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}
