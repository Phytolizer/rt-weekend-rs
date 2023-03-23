use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::vec3::Vec3;

use super::HitRecord;
use super::Hittable;

pub struct Translate {
    ptr: Box<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(ptr: Box<dyn Hittable>, offset: Vec3) -> Self {
        Self { ptr, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_r = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
        let rec = self.ptr.hit(&moved_r, t_min, t_max)?;

        Some(HitRecord::new(
            rec.p + self.offset,
            rec.normal,
            rec.t,
            rec.u,
            rec.v,
            &moved_r,
            rec.mat_ptr,
        ))
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<Aabb> {
        let out_box = self.ptr.bounding_box(time0, time1)?;

        Some(Aabb::new(
            out_box.min + self.offset,
            out_box.max + self.offset,
        ))
    }
}
