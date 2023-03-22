use std::f64;
use std::sync::Arc;

use rand::Rng;

use crate::aabb::Aabb;
use crate::onb::Onb;
use crate::vec3::Vec3;
use crate::{material::Material, ray::Ray, Point3};

use super::{HitRecord, Hittable};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat_ptr,
        }
    }

    pub(super) fn get_uv(p: Point3) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + f64::consts::PI;

        (phi / f64::consts::TAU, theta / f64::consts::PI)
    }
}

fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen_range(0.0..1.0);
    let r2 = rng.gen_range(0.0..1.0);
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = f64::consts::TAU * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    Vec3::new(x, y, z)
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let half_b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let root = {
            let mut root = (-half_b - sqrtd) / a;
            if !(t_min..t_max).contains(&root) {
                root = (-half_b + sqrtd) / a;
                if !(t_min..t_max).contains(&root) {
                    return None;
                }
            }
            root
        };

        let t = root;
        let p = ray.at(t);
        let normal = (p - self.center) / self.radius;
        let (u, v) = Self::get_uv(normal);
        Some(HitRecord::new(
            p,
            normal,
            t,
            u,
            v,
            ray,
            self.mat_ptr.clone(),
        ))
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }

    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        if self
            .hit(&Ray::new(o, v, 0.0), 0.001, f64::INFINITY)
            .is_none()
        {
            return 0.0;
        };

        let cos_theta_max =
            (1.0 - self.radius * self.radius / (self.center - o).magnitude_squared()).sqrt();
        let solid_angle = f64::consts::TAU * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }

    fn random(&self, o: Vec3) -> Vec3 {
        let direction = self.center - o;
        let distance_squared = direction.magnitude_squared();
        let uvw = Onb::from_w(&direction);
        uvw.local(random_to_sphere(self.radius, distance_squared))
    }
}
