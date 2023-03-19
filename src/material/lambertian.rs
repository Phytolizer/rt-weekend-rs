use crate::{hittable::HitRecord, random_unit_vector, ray::Ray, Color};

use super::{Material, ScatterRecord};

pub struct Lambertian {
    color: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = {
            let mut scatter_direction = rec.normal + random_unit_vector();
            if scatter_direction.near_zero() {
                scatter_direction = rec.normal;
            }
            scatter_direction
        };

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.color;

        Some(ScatterRecord {
            attenuation,
            scattered,
        })
    }
}
