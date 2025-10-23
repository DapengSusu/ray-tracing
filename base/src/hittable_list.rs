use std::sync::Arc;

use crate::{HitRecord, Hittable, interval::Interval, ray::Ray};

/// A collection of Hittable objects.
#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
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
    pub fn from_hittable(hittable: Arc<dyn Hittable>) -> Self {
        Self {
            objects: vec![hittable],
        }
    }

    /// Creates a new `HittableList` containing multiple `Hittable` objects.
    pub fn from_hittables(objects: Vec<Arc<dyn Hittable>>) -> Self {
        Self { objects }
    }

    /// Adds a `Hittable` object to the list.
    pub fn add(&mut self, hittable: Arc<dyn Hittable>) {
        self.objects.push(hittable);
    }

    /// Returns true if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    /// Returns the number of `Hittable` objects in the list.
    pub fn len(&self) -> usize {
        self.objects.len()
    }

    /// Clears the list of all `Hittable` objects.
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
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
