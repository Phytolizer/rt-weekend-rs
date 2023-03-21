use crate::random_vec_in_unit_sphere;
use crate::{hittable::HitRecord, ray::Ray, Color};

use super::{Material, ScatterRecord};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = ray.direction.normalize().reflect(&rec.normal);

        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * random_vec_in_unit_sphere(),
            ray.time,
        );
        let attenuation = self.albedo;

        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation,
                scattered,
            })
        } else {
            None
        }
    }
}
