use crate::Point3;
use crate::{hittable::HitRecord, ray::Ray, Color};

pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

pub struct ScatterRecord {
    pub albedo: Color,
    pub scattered: Ray,
    pub pdf: f64,
}

pub trait Material: Send + Sync {
    fn scatter(&self, _ray: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
    fn emitted(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        Color::zeros()
    }
}
