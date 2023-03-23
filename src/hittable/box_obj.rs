use std::sync::Arc;

use crate::aabb::Aabb;
use crate::material::Material;
use crate::ray::Ray;
use crate::Point3;

use super::aa_rect::XyRect;
use super::aa_rect::XzRect;
use super::aa_rect::YzRect;
use super::hittable_list::HittableList;
use super::HitRecord;
use super::Hittable;

pub struct BoxObj {
    min: Point3,
    max: Point3,
    sides: HittableList,
}

impl BoxObj {
    pub fn new(min: Point3, max: Point3, mat_ptr: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        sides.add(Arc::new(XyRect::new(
            min.x,
            max.x,
            min.y,
            max.y,
            max.z,
            mat_ptr.clone(),
        )));
        sides.add(Arc::new(XyRect::new(
            min.x,
            max.x,
            min.y,
            max.y,
            min.z,
            mat_ptr.clone(),
        )));

        sides.add(Arc::new(XzRect::new(
            min.x,
            max.x,
            min.z,
            max.z,
            max.y,
            mat_ptr.clone(),
        )));
        sides.add(Arc::new(XzRect::new(
            min.x,
            max.x,
            min.z,
            max.z,
            min.y,
            mat_ptr.clone(),
        )));

        sides.add(Arc::new(YzRect::new(
            min.y,
            max.y,
            min.z,
            max.z,
            max.x,
            mat_ptr.clone(),
        )));
        sides.add(Arc::new(YzRect::new(
            min.y,
            max.y,
            min.z,
            max.z,
            min.x,
            mat_ptr.clone(),
        )));

        Self { min, max, sides }
    }
}

impl Hittable for BoxObj {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<Aabb> {
        Some(Aabb::new(self.min, self.max))
    }
}
