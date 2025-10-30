// use std::f64::consts::PI;

use crate::{
    Interval, Point3, Ray,
    hittable::{HitRecord, Hittable},
};

#[derive(Debug, Default, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    /// Create stationary sphere
    pub fn new(static_center: Point3, radius: f64) -> Self {
        Sphere {
            center: static_center,
            radius: radius.max(0.),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = r.direction.dot(&oc);
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
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;

        Some(
            HitRecord::builder()
                .set_t(t)
                .set_p(p)
                .set_face_normal(r, outward_normal),
        )
    }
}

// /// Takes points on the unit sphere centered at the origin, and computes (u, v)
// ///
// /// * p: a given point on the sphere of radius one, centered at the origin.
// ///
// /// # Returns
// ///
// /// (u, v)
// ///
// /// * u: returned value [0,1] of angle around the Y axis from X=-1.
// /// * v: returned value [0,1] of angle from Y=-1 to Y=+1.
// ///
// /// # Tip
// ///
// /// <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
// ///
// /// <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
// ///
// /// <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
// pub fn get_sphere_uv(p: &Point3) -> UvCoord {
//     let theta = (-p.y).acos();
//     let phi = (-p.z).atan2(p.x) + PI;

//     let u = phi / (2. * PI);
//     let v = theta / PI;

//     UvCoord::new(u, v)
// }
