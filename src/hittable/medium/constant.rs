use std::f64;
use std::sync::Arc;

use rand::Rng;

use crate::aabb::Aabb;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::material::isotropic::Isotropic;
use crate::material::Material;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;
use crate::Color;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, albedo: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(albedo)),
        }
    }

    pub fn new_color(boundary: Arc<dyn Hittable>, density: f64, color: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new_color(color)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        const enable_debug: bool = false;
        let debugging = enable_debug && rand::thread_rng().gen_ratio(1, 100000);

        let mut rec1 = self.boundary.hit(ray, f64::NEG_INFINITY, f64::INFINITY)?;
        let mut rec2 = self.boundary.hit(ray, rec1.t + 0.0001, f64::INFINITY)?;

        if debugging {
            eprintln!("\nt_min={}, t_max={}", rec1.t, rec2.t);
        }

        rec1.t = rec1.t.max(t_min);
        rec2.t = rec2.t.min(t_max);

        if rec1.t >= rec2.t {
            return None;
        }

        rec1.t = rec1.t.max(0.0);

        let ray_length = ray.direction.magnitude();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density
            * rand::thread_rng()
                .gen_range(0.0f64..1.0)
                .log(f64::consts::E);

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let p = ray.at(t);

        if debugging {
            dbg!(hit_distance);
            dbg!(t);
            dbg!(p);
        }

        // arbitrary
        let normal = Vec3::new(1.0, 0.0, 0.0);
        let front_face = true;

        let mat_ptr = self.phase_function.clone();

        Some(HitRecord {
            p,
            normal,
            t,
            u: 0.0,
            v: 0.0,
            front_face,
            mat_ptr,
        })
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}
