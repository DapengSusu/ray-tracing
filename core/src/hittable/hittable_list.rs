use std::ops::{Deref, DerefMut};

use crate::prelude::*;

/// A collection of Hittable objects.
#[derive(Debug, Default, Clone)]
pub struct HittableList {
    pub objects: Vec<HittableObject>,
    bounding_box: AABB,
}

impl HittableList {
    /// Creates a new empty `HittableList`.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bounding_box: AABB::default(),
        }
    }

    /// Creates a new `HittableList` with a specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            objects: Vec::with_capacity(capacity),
            bounding_box: AABB::default(),
        }
    }

    /// Creates a new `HittableList` containing a `Hittable` object.
    pub fn from_hittable(hittable: HittableObject) -> Self {
        let bounding_box = AABB::from_boxes(&AABB::default(), hittable.bounding_box());

        Self {
            objects: vec![hittable],
            bounding_box,
        }
    }

    /// Creates a new `HittableList` containing multiple `Hittable` objects.
    pub fn from_hittables(objects: Vec<HittableObject>) -> Self {
        let bounding_box: AABB = objects
            .iter()
            .map(|hittable| hittable.bounding_box().to_owned())
            .sum();

        Self {
            objects,
            bounding_box,
        }
    }

    /// Adds a `Hittable` object to the list.
    pub fn add(&mut self, hittable: HittableObject) {
        self.bounding_box = AABB::from_boxes(&self.bounding_box, hittable.bounding_box());
        self.objects.push(hittable);
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

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

impl Deref for HittableList {
    type Target = Vec<HittableObject>;

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
    use std::sync::Arc;

    use super::*;

    #[test]
    fn hittable_list_deref_should_work() {
        let mut list = HittableList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);

        list.add(HittableObject::Sphere(Sphere::new(
            Point3::zero(),
            1.,
            Arc::new(MaterialType::Lambertian(Lambertian::from_color(
                Color::zero(),
            ))),
        )));
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);

        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }
}
