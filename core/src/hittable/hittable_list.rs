use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::{
    Interval, Ray,
    hittable::{HitRecord, Hittable, HittableObject},
};

/// A collection of Hittable objects.
#[derive(Debug, Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<HittableObject>>,
}

impl HittableList {
    /// Creates a new empty `HittableList`.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// Creates a new `HittableList` with a specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            objects: Vec::with_capacity(capacity),
        }
    }

    /// Creates a new `HittableList` containing a `Hittable` object.
    pub fn from_hittable(hittable: Arc<HittableObject>) -> Self {
        Self {
            objects: vec![hittable],
        }
    }

    /// Creates a new `HittableList` containing multiple `Hittable` objects.
    pub fn from_hittables(objects: Vec<Arc<HittableObject>>) -> Self {
        Self { objects }
    }

    /// Adds a `Hittable` object to the list.
    pub fn add(&mut self, hittable: Arc<HittableObject>) {
        self.objects.push(hittable);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut hit_record = None;

        for hittable in &self.objects {
            let r_t = Interval::new(ray_t.min, closest_so_far);
            if let Some(hit) = hittable.hit(r, &r_t) {
                closest_so_far = hit.t;
                hit_record = Some(hit);
            }
        }

        hit_record
    }
}

impl Deref for HittableList {
    type Target = Vec<Arc<HittableObject>>;

    fn deref(&self) -> &Self::Target {
        &self.objects
    }
}

impl DerefMut for HittableList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.objects
    }
}

#[cfg(test)]
mod tests {
    use crate::{Point3, Sphere};

    use super::*;

    #[test]
    fn hittable_list_deref_should_work() {
        let mut list = HittableList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);

        let sphere = Sphere::new(Point3::ZERO, 1.);
        list.add(Arc::new(HittableObject::Sphere(sphere)));
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);

        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }
}
