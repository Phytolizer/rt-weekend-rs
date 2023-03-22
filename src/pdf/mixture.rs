use std::sync::Arc;

use rand::Rng;

use crate::vec3::Vec3;

use super::Pdf;

pub struct MixturePdf {
    p: [Arc<dyn Pdf>; 2],
}

impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        Self { p: [p0, p1] }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        if rand::thread_rng().gen_bool(0.5) {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
