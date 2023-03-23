use std::f32;
use std::sync::Arc;

use rand::Rng;

use crate::pdf::cosine::CosinePdf;
use crate::texture::solid_color::SolidColor;
use crate::texture::Texture;
use crate::vec3::Vec3;
use crate::{hittable::HitRecord, ray::Ray, Color};

use super::{Material, ScatterRecord, ScatterType};

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
    let r1 = rand::thread_rng().gen_range(0.0f32..1.0);
    let r2 = rand::thread_rng().gen_range(0.0f32..1.0);
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * f32::consts::PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            attenuation: self.albedo.value(rec.u, rec.v, rec.p),
            scattered: ScatterType::Diffuse(Arc::new(CosinePdf::new(&rec.normal))),
        })
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f32 {
        let cosine = rec.normal.dot(&scattered.direction.normalize());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / f32::consts::PI
        }
    }
}
