use std::cmp::Ordering;
use std::sync::Arc;

use rand::Rng;

use crate::aabb::Aabb;
use crate::ray::Ray;

use super::HitRecord;
use super::Hittable;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bb: Aabb,
}

impl BvhNode {
    pub fn new(list: &[Arc<dyn Hittable>], time0: f32, time1: f32) -> Self {
        let mut objects = list.to_vec();

        let axis = rand::thread_rng().gen_range(0..3);
        let comparator = |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| -> Ordering {
            let box_a = a.bounding_box(0.0, 0.0).unwrap();
            let box_b = b.bounding_box(0.0, 0.0).unwrap();
            box_a.min[axis].total_cmp(&box_b.min[axis])
        };

        let (left, right) = match objects.len() {
            1 => (objects[0].clone(), objects[0].clone()),
            2 => {
                if comparator(&objects[0], &objects[1]) == Ordering::Less {
                    (objects[0].clone(), objects[1].clone())
                } else {
                    (objects[1].clone(), objects[0].clone())
                }
            }
            _ => {
                objects.sort_unstable_by(comparator);
                let mid = objects.len() / 2;
                let left = Arc::new(BvhNode::new(&objects[..mid], time0, time1));
                let right = Arc::new(BvhNode::new(&objects[mid + 1..], time0, time1));
                (left as Arc<dyn Hittable>, right as Arc<dyn Hittable>)
            }
        };

        let box_left = left.bounding_box(time0, time1).unwrap();
        let box_right = right.bounding_box(time0, time1).unwrap();

        Self {
            left,
            right,
            bb: Aabb::surrounding(&box_left, &box_right),
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.bb.hit(ray, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(ray, t_min, t_max);
        let hit_right = self.right.hit(
            ray,
            t_min,
            hit_left.as_ref().map(|rec| rec.t).unwrap_or(t_max),
        );
        hit_right.or(hit_left)
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<Aabb> {
        Some(self.bb.clone())
    }
}
