use std::sync::Arc;

use crate::prelude::*;

pub struct Quad {
    /// Q: The starting corner.
    q: Point3,
    /// A vector representing the first side.
    /// `Q+u` gives one of the corners adjacent to `Q`.
    u: Vec3,
    /// A vector representing the second side.
    /// `Q+v` gives the other corner adjacent to `Q`.
    v: Vec3,
    /// The vector `w` is constant for a given quadrilateral.
    w: Vec3,
    /// Material of the quad.
    material: Arc<dyn Material>,
    /// Axis-aligned bounding box of the quad.
    bounding_box: AABB,
    /// Normal vector
    normal: Vec3,
    /// `D` constant
    d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        let bounding_box = new_bounding_box(&q, &u, &v);
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
            bounding_box,
            normal,
            d,
        }
    }
}

impl Hittable for Quad {
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

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = ray.at(t);

        let mut hit_record = HitRecord::builder()
            .set_t(t)
            .set_p(intersection)
            .set_material(Some(self.material.clone()))
            .set_face_normal(ray, self.normal);

        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));

        if !is_interior(&mut hit_record, alpha, beta) {
            return None;
        }

        // Ray hits the 2D shape; set the rest of the hit record and return true.

        Some(hit_record)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

fn is_interior(hit: &mut HitRecord, a: f64, b: f64) -> bool {
    let unit_interval = Interval::new(0., 1.);
    // Given the hit point in plane coordinates, return false if it is outside the
    // primitive, otherwise set the hit record UV coordinates and return true.

    if !unit_interval.contains(a) || !unit_interval.contains(b) {
        return false;
    }

    hit.uv = UvCoord::new(a, b);

    true
}

// Compute the bounding box of all four vertices.
fn new_bounding_box(q: &Point3, u: &Vec3, v: &Vec3) -> AABB {
    let bbox_diagonal1 = AABB::with_points(*q, *q + *u + *v);
    let bbox_diagonal2 = AABB::with_points(*q + *u, *q + *v);

    AABB::from_boxes(&bbox_diagonal1, &bbox_diagonal2)
}
