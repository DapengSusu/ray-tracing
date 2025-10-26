use std::{f64::consts::PI, sync::Arc};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    material: Arc<MaterialType>,
    bounding_box: AABB,
}

impl Sphere {
    /// Create stationary sphere
    pub fn new(static_center: Point3, radius: f64, material: MaterialType) -> Self {
        let rvec = Vec3::with_isotropic(radius);

        Sphere {
            center: Ray::new(static_center, Vec3::zero()),
            radius: radius.max(0.),
            material: Arc::new(material),
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
        material: MaterialType,
    ) -> Self {
        let center = Ray::new(center_original, center_end - center_original);
        let rvec = Vec3::with_isotropic(radius);
        let box0 = AABB::with_points(center.at(0.) - rvec, center.at(0.) + rvec);
        let box1 = AABB::with_points(center.at(1.) - rvec, center.at(1.) + rvec);

        Sphere {
            center,
            radius: radius.max(0.),
            material: Arc::new(material),
            bounding_box: AABB::from_boxes(&box0, &box1),
        }
    }

    /// Takes points on the unit sphere centered at the origin, and computes (u, v)
    ///
    /// * p: a given point on the sphere of radius one, centered at the origin.
    ///
    /// # Returns
    ///
    /// (u, v)
    ///
    /// * u: returned value [0,1] of angle around the Y axis from X=-1.
    /// * v: returned value [0,1] of angle from Y=-1 to Y=+1.
    ///
    /// # Tip
    ///
    /// <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    ///
    /// <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    ///
    /// <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
    pub fn get_sphere_uv(p: &Point3) -> UvCoord {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        let u = phi / (2. * PI);
        let v = theta / PI;

        UvCoord::new(u, v)
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
            .set_uv(Self::get_sphere_uv(&outward_normal))
            .set_face_normal(ray, outward_normal)
            .set_material(self.material.clone());

        Some(hit_record)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}
