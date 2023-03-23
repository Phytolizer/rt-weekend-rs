use std::sync::Arc;

use crate::aabb::Aabb;
use crate::{material::Material, ray::Ray, Point3, Vec3};

pub mod aa_rect;
pub mod box_obj;
pub mod bvh_node;
pub mod flip_face;
pub mod hittable_list;
pub mod medium;
pub mod moving_sphere;
pub mod rotate;
pub mod sphere;
pub mod translate;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
    pub mat_ptr: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        outward_normal: Vec3,
        t: f32,
        u: f32,
        v: f32,
        r: &Ray,
        mat_ptr: Arc<dyn Material>,
    ) -> Self {
        let front_face = r.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            p,
            normal,
            t,
            u,
            v,
            front_face,
            mat_ptr,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<Aabb>;
    fn pdf_value(&self, _o: Point3, _v: Vec3) -> f32 {
        0.0
    }
    fn random(&self, _o: Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}
