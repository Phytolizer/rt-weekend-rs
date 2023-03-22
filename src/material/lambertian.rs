use std::f64;
use std::sync::Arc;

use rand::Rng;

use crate::onb::Onb;
use crate::texture::solid_color::SolidColor;
use crate::texture::Texture;
use crate::vec3::Vec3;
use crate::{hittable::HitRecord, ray::Ray, Color};

use super::{Material, ScatterRecord};

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn new_color(albedo: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(albedo)),
        }
    }
}

fn random_cosine_direction() -> Vec3 {
    let r1 = rand::thread_rng().gen_range(0.0f64..1.0);
    let r2 = rand::thread_rng().gen_range(0.0f64..1.0);
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * f64::consts::PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let uvw = Onb::from_w(&rec.normal);
        let direction = uvw.local(random_cosine_direction());
        let scattered = Ray::new(rec.p, direction.normalize(), ray.time);
        let albedo = self.albedo.value(rec.u, rec.v, rec.p);
        let pdf = uvw.w().dot(&scattered.direction) / f64::consts::PI;

        Some(ScatterRecord {
            albedo,
            scattered,
            pdf,
        })
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = rec.normal.dot(&scattered.direction.normalize());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / f64::consts::PI
        }
    }
}
