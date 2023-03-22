use crate::random_vec_in_unit_sphere;
use crate::{hittable::HitRecord, ray::Ray, Color};

use super::{Material, ScatterRecord, ScatterType};

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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = r_in.direction.normalize().reflect(&rec.normal);

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: ScatterType::Specular(Ray::new(
                rec.p,
                reflected + self.fuzz * random_vec_in_unit_sphere(),
                0.0,
            )),
        })
    }
}
