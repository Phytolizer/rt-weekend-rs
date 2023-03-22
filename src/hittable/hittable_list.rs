use std::sync::Arc;

use crate::aabb::Aabb;

use super::{HitRecord, Hittable};

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub(crate) fn children(&self) -> &[Arc<dyn Hittable>] {
        self.objects.as_slice()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let mut temp_box = None;

        for object in &self.objects {
            if let Some(bb) = object.bounding_box(time0, time1) {
                temp_box = Some(temp_box.map(|tb| Aabb::surrounding(&tb, &bb)).unwrap_or(bb));
            }
        }

        temp_box
    }
}
