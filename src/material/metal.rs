use crate::{hittable::HitRecord, ray::Ray, Color};

use super::{Material, ScatterRecord};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = ray.direction.normalize().reflect(&rec.normal);

        let scattered = Ray::new(rec.p, reflected);
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
