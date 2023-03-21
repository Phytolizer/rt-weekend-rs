use std::sync::Arc;

use crate::texture::solid_color::SolidColor;
use crate::texture::Texture;
use crate::{hittable::HitRecord, random_unit_vector, ray::Ray, Color};

use super::{Material, ScatterRecord};

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(albedo)),
        }
    }

    pub fn new_tex(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = {
            let mut scatter_direction = rec.normal + random_unit_vector();
            if scatter_direction.near_zero() {
                scatter_direction = rec.normal;
            }
            scatter_direction
        };

        let scattered = Ray::new(rec.p, scatter_direction, ray.time);
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);

        Some(ScatterRecord {
            attenuation,
            scattered,
        })
    }
}
