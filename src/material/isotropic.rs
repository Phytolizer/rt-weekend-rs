use std::sync::Arc;

use crate::hittable::HitRecord;
use crate::random_vec_in_unit_sphere;
use crate::ray::Ray;
use crate::texture::solid_color::SolidColor;
use crate::texture::Texture;
use crate::Color;

use super::Material;
use super::ScatterRecord;
use super::ScatterType;

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn new_color(color: Color) -> Self {
        Self::new(Arc::new(SolidColor::new(color)))
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            attenuation: self.albedo.value(rec.u, rec.v, rec.p),
            scattered: ScatterType::Specular(Ray::new(
                rec.p,
                random_vec_in_unit_sphere(),
                ray.time,
            )),
        })
    }
}
