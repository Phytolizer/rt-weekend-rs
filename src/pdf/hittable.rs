use std::sync::Arc;

use crate::{hittable::Hittable, vec3::Vec3, Point3};

use super::Pdf;

pub struct HittablePdf {
    origin: Point3,
    ptr: Arc<dyn Hittable>,
}

impl HittablePdf {
    pub fn new(ptr: Arc<dyn Hittable>, origin: Point3) -> Self {
        Self { origin, ptr }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: Vec3) -> f64 {
        self.ptr.pdf_value(self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.random(self.origin)
    }
}
