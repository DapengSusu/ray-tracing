mod bvh;
mod hittable_list;
mod sphere;
mod triangle;

pub mod quad;

pub use bvh::BvhNode;
pub use hittable_list::HittableList;
pub use quad::Quad;
pub use sphere::Sphere;
pub use triangle::Triangle;

use std::sync::Arc;

use crate::prelude::*;

/// Trait for objects that can be hit by rays.
pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> &AABB;
}

/// Different types of hittable objects.
#[derive(Debug, Clone)]
pub enum HittableObject {
    HittableList(HittableList),
    Sphere(Sphere),
    BvhNode(BvhNode),
    Quad(Quad),
    Triangle(Triangle),
}

impl HittableObject {
    pub fn new_list(objects: Vec<HittableObject>) -> Self {
        Self::HittableList(HittableList::from_hittables(objects))
    }

    pub fn add(&mut self, hittable: HittableObject) {
        match self {
            Self::HittableList(list) => list.add(hittable),
            _ => panic!("Unsupported 'add' operation"),
        }
    }

    pub fn new_sphere(static_center: Point3, radius: f64, material: MaterialType) -> Self {
        HittableObject::Sphere(Sphere::new(static_center, radius, material))
    }

    pub fn new_sphere_moving(
        center_original: Point3,
        center_end: Point3,
        radius: f64,
        material: MaterialType,
    ) -> Self {
        HittableObject::Sphere(Sphere::new_moving(
            center_original,
            center_end,
            radius,
            material,
        ))
    }

    pub fn new_bvh_node(list: HittableList) -> Self {
        HittableObject::BvhNode(BvhNode::from_hittable_list(list))
    }

    pub fn new_quad(q: Point3, u: Vec3, v: Vec3, material: MaterialType) -> Self {
        HittableObject::Quad(Quad::new(q, u, v, Arc::new(material)))
    }

    pub fn new_triangle(q: Point3, u: Vec3, v: Vec3, material: MaterialType) -> Self {
        HittableObject::Triangle(Triangle::new(q, u, v, Arc::new(material)))
    }
}

impl Hittable for HittableObject {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            HittableObject::HittableList(list) => list.hit(ray, ray_t),
            HittableObject::Sphere(sphere) => sphere.hit(ray, ray_t),
            HittableObject::BvhNode(node) => node.hit(ray, ray_t),
            HittableObject::Quad(quad) => quad.hit(ray, ray_t),
            HittableObject::Triangle(triangle) => triangle.hit(ray, ray_t),
        }
    }

    fn bounding_box(&self) -> &AABB {
        match self {
            HittableObject::HittableList(list) => list.bounding_box(),
            HittableObject::Sphere(sphere) => sphere.bounding_box(),
            HittableObject::BvhNode(node) => node.bounding_box(),
            HittableObject::Quad(quad) => quad.bounding_box(),
            HittableObject::Triangle(triangle) => triangle.bounding_box(),
        }
    }
}

#[derive(Default)]
pub struct HitRecord {
    pub t: f64,
    /// (u,v) surface coordinates of the ray-object hit point.
    pub uv: UvCoord,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Option<Arc<MaterialType>>,
}

impl HitRecord {
    /// Builds a default `HitRecord`.
    pub fn builder() -> Self {
        Self::default()
    }

    /// Sets the parameter `t` of the hit record.
    pub fn set_t(mut self, t: f64) -> Self {
        self.t = t;
        self
    }

    /// Sets the parameter `uv` of the hit record.
    pub fn set_uv(mut self, u: f64, v: f64) -> Self {
        self.uv = UvCoord::new(u, v);
        self
    }

    /// Sets the parameter `p` of the hit record.
    pub fn set_p(mut self, p: Point3) -> Self {
        self.p = p;
        self
    }

    /// Sets the parameter `material` of the hit record.
    pub fn set_material(mut self, material: Arc<MaterialType>) -> Self {
        self.material = Some(material);
        self
    }

    /// Sets the face normal based on the given ray and outward normal.
    ///
    /// # Note
    ///
    /// the parameter `outward_normal` is assumed to have unit length.
    pub fn set_face_normal(mut self, ray: &Ray, outward_normal: Vec3) -> Self {
        self.front_face = ray.direction.dot(&outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
        self
    }
}
