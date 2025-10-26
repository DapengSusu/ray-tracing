use std::sync::Arc;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Triangle {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    material: Arc<MaterialType>,
    bounding_box: AABB,
    normal: Vec3,
    d: f64,
}

impl Triangle {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: Arc<MaterialType>) -> Self {
        let n = vec3::cross(&u, &v);
        let normal = n.to_unit();
        let d = vec3::dot(&normal, &q);
        let w = n / n.dot_self();

        Self {
            q,
            u,
            v,
            w,
            material,
            bounding_box: AABB::default(),
            normal,
            d,
        }
        .update_bounding_box(&q, &u, &v)
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = vec3::dot(&self.normal, &ray.direction);

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return None;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - vec3::dot(&self.normal, &ray.origin)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = ray.at(t);
        let hit_record = HitRecord::builder()
            .set_t(t)
            .set_p(intersection)
            .set_material(self.material.clone())
            .set_face_normal(ray, self.normal);

        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));

        Self::is_interior(hit_record, alpha, beta)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

impl PlaneFigure for Triangle {
    fn update_bounding_box(mut self, q: &Point3, u: &Point3, v: &Point3) -> Self {
        let bbox_diagonal1 = AABB::with_points(*q, *q + *u + *v);
        let bbox_diagonal2 = AABB::with_points(*q + *u, *q + *v);

        self.bounding_box
            .replace(AABB::from_boxes(&bbox_diagonal1, &bbox_diagonal2));

        self
    }

    fn is_interior(hit: HitRecord, a: f64, b: f64) -> Option<HitRecord> {
        if a <= 0. || b <= 0. || a + b >= 1. {
            return None;
        }

        Some(hit.set_uv(a, b))
    }
}
