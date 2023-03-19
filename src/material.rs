use crate::{hittable::HitRecord, ray::Ray, Color};

pub mod lambertian;
pub mod metal;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
}
