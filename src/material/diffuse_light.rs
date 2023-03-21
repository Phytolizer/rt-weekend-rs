use std::sync::Arc;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::solid_color::SolidColor;
use crate::texture::Texture;
use crate::Color;

use super::Material;
use super::ScatterRecord;

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }

    pub fn new_color(emit: Color) -> Self {
        Self::new(Arc::new(SolidColor::new(emit)))
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: crate::Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
