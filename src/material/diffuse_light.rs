use std::sync::Arc;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::solid_color::SolidColor;
use crate::texture::Texture;
use crate::Color;
use crate::Point3;

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

    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: Point3) -> Color {
        if rec.front_face {
            self.emit.value(u, v, p)
        } else {
            Color::zeros()
        }
    }
}
