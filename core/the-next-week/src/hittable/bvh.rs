use std::{cmp::Ordering, sync::Arc};

use crate::prelude::*;

/// A BVH is also going to be a hittable — just like lists of hittables.
/// It’s really a container, but it can respond to the query “does this ray hit you?”.
#[derive(Default)]
pub struct BvhNode {
    left: Option<Arc<dyn Hittable>>,
    right: Option<Arc<dyn Hittable>>,
    bounding_box: AABB,
}

impl BvhNode {
    pub fn from_hittable_list(list: HittableList) -> Self {
        let len = list.len();
        Self::from_hittables(list.objects, 0, len)
    }

    pub fn from_hittables(mut objects: Vec<Arc<dyn Hittable>>, begin: usize, end: usize) -> Self {
        let mut bvh_node = Self::default();

        // Build the bounding box of the span of source objects.
        (begin..end).for_each(|i| {
            bvh_node.bounding_box += objects[i].bounding_box().to_owned();
        });

        let object_span = end - begin;
        if object_span == 1 {
            bvh_node.left = Some(objects[begin].clone());
            bvh_node.right = Some(objects[begin].clone());
        } else if object_span == 2 {
            bvh_node.left = Some(objects[begin].clone());
            bvh_node.right = Some(objects[begin + 1].clone());
        } else {
            match bvh_node.bounding_box.longest_axis() {
                0 => objects[begin..end].sort_by(box_x_compare),
                1 => objects[begin..end].sort_by(box_y_compare),
                _ => objects[begin..end].sort_by(box_z_compare),
            };
            let mid = begin + object_span / 2;
            let left = Self::from_hittables(objects.clone(), begin, mid);
            let right = Self::from_hittables(objects.clone(), mid, end);
            bvh_node.left = Some(Arc::new(left));
            bvh_node.right = Some(Arc::new(right));
        }

        bvh_node
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, ray_t) {
            return None;
        }

        let left_hit = self.left.as_ref().and_then(|l| l.hit(ray, ray_t));
        let right_hit = self.right.as_ref().and_then(|r| {
            r.hit(
                ray,
                Interval::new(
                    ray_t.min,
                    match &left_hit {
                        Some(rec) => rec.t,
                        None => ray_t.max,
                    },
                ),
            )
        });

        right_hit.or(left_hit)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: u8) -> Ordering {
    let a_axis_interval = a.bounding_box()[axis];
    let b_axis_interval = b.bounding_box()[axis];

    a_axis_interval.min.total_cmp(&b_axis_interval.min)
}

fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}
