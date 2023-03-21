use crate::Point3;
use crate::{hittable::HitRecord, ray::Ray, Color};

pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
    fn emitted(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        Color::zeros()
    }
}
