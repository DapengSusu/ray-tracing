#![allow(unused)]

use ray_tracing_core::{
    Point3, Vec3, aabb::AABB, hittable::HitRecord, interval::Interval, ray::Ray,
};

enum HittableObject {
    ObjectList(HittableObjectList),
    Sphere(Sphere),
    Quad(Quad),
}

trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

impl Hittable for HittableObject {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            HittableObject::ObjectList(list) => list.hit(ray, ray_t),
            HittableObject::Sphere(sphere) => sphere.hit(ray, ray_t),
            HittableObject::Quad(quad) => quad.hit(ray, ray_t),
        }
    }
}

#[derive(Default)]
pub(crate) struct HittableObjectList {
    pub(crate) objects: Vec<HittableObject>,
}

impl Hittable for HittableObjectList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut hit_record = None;

        for hittable in &self.objects {
            if let Some(hit) = hittable.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit.t;
                hit_record = Some(hit);
            }
        }

        hit_record
    }
}

#[derive(Default)]
struct Sphere {
    center: Ray,
    radius: f64,
    bounding_box: AABB,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        None
    }
}

#[derive(Default)]
struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    bounding_box: AABB,
    normal: Vec3,
    d: f64,
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        None
    }
}

fn main() {
    let mut list = HittableObjectList::default();

    list.objects.push(HittableObject::Sphere(Sphere::default()));
    list.objects.push(HittableObject::Quad(Quad::default()));
}
