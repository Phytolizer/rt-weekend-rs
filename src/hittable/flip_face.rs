use crate::{aabb::Aabb, ray::Ray};

use super::{HitRecord, Hittable};

pub struct FlipFace {
    ptr: Box<dyn Hittable>,
}

impl FlipFace {
    pub fn new(ptr: Box<dyn Hittable>) -> Self {
        Self { ptr }
    }
}

impl Hittable for FlipFace {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let rec = self.ptr.hit(ray, t_min, t_max)?;

        Some(HitRecord {
            p: rec.p,
            normal: rec.normal,
            t: rec.t,
            u: rec.u,
            v: rec.v,
            front_face: !rec.front_face,
            mat_ptr: rec.mat_ptr,
        })
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<Aabb> {
        self.ptr.bounding_box(time0, time1)
    }
}
