use std::sync::Arc;

use crate::{aabb::Aabb, vec3::Vec3, Point3};

use super::{HitRecord, Hittable};

use rand::seq::SliceRandom;

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
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<Aabb> {
        let mut temp_box = None;

        for object in &self.objects {
            if let Some(bb) = object.bounding_box(time0, time1) {
                temp_box = Some(temp_box.map(|tb| Aabb::surrounding(&tb, &bb)).unwrap_or(bb));
            }
        }

        temp_box
    }

    fn pdf_value(&self, o: Point3, v: Vec3) -> f32 {
        self.objects
            .iter()
            .map(|obj| obj.pdf_value(o, v))
            .sum::<f32>()
            / self.objects.len() as f32
    }

    fn random(&self, o: Vec3) -> Vec3 {
        self.objects
            .choose(&mut rand::thread_rng())
            .map(|obj| obj.random(o))
            .unwrap_or(Vec3::zeros())
    }
}
