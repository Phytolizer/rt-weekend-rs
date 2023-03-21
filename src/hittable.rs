use std::sync::Arc;

use crate::aabb::Aabb;
use crate::{material::Material, ray::Ray, Point3, Vec3};

pub mod aa_rect;
pub mod box_obj;
pub mod bvh_node;
pub mod hittable_list;
pub mod moving_sphere;
pub mod rotate;
pub mod sphere;
pub mod translate;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat_ptr: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        outward_normal: Vec3,
        t: f64,
        u: f64,
        v: f64,
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}
