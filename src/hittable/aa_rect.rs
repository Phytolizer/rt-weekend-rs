use std::sync::Arc;

use rand::Rng;

use crate::aabb::Aabb;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::Point3;

use super::HitRecord;
use super::Hittable;

pub struct XyRect {
    mp: Arc<dyn Material>,
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
}

impl XyRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, mp: Arc<dyn Material>) -> Self {
        Self {
            mp,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl Hittable for XyRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if !(t_min..t_max).contains(&t) {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if !(self.x0..self.x1).contains(&x) || !(self.y0..self.y1).contains(&y) {
            return None;
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let p = ray.at(t);

        Some(HitRecord::new(
            p,
            outward_normal,
            t,
            u,
            v,
            ray,
            self.mp.clone(),
        ))
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

pub struct XzRect {
    mp: Arc<dyn Material>,
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl XzRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, mp: Arc<dyn Material>) -> Self {
        Self {
            mp,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for XzRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if !(t_min..t_max).contains(&t) {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if !(self.x0..self.x1).contains(&x) || !(self.z0..self.z1).contains(&z) {
            return None;
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        let p = ray.at(t);

        Some(HitRecord::new(
            p,
            outward_normal,
            t,
            u,
            v,
            ray,
            self.mp.clone(),
        ))
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }

    fn pdf_value(&self, o: Point3, v: Vec3) -> f32 {
        let rec = match self.hit(&Ray::new(o, v, 0.0), 0.001, f32::INFINITY) {
            Some(rec) => rec,
            None => return 0.0,
        };

        let area = (self.x1 - self.x0) * (self.z1 - self.z0);
        let distance_squared = rec.t * rec.t * v.magnitude_squared();
        let cosine = (v.dot(&rec.normal) / v.magnitude()).abs();

        distance_squared / (cosine * area)
    }

    fn random(&self, o: Vec3) -> Vec3 {
        let mut rng = rand::thread_rng();
        let random_point = Point3::new(
            rng.gen_range(self.x0..self.x1),
            self.k,
            rng.gen_range(self.z0..self.z1),
        );
        random_point - o
    }
}

pub struct YzRect {
    mp: Arc<dyn Material>,
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl YzRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, mp: Arc<dyn Material>) -> Self {
        Self {
            mp,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for YzRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if !(t_min..t_max).contains(&t) {
            return None;
        }
        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;
        if !(self.y0..self.y1).contains(&y) || !(self.z0..self.z1).contains(&z) {
            return None;
        }
        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        let p = ray.at(t);

        Some(HitRecord::new(
            p,
            outward_normal,
            t,
            u,
            v,
            ray,
            self.mp.clone(),
        ))
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
