use std::f32;

use rand::Rng;

use crate::{onb::Onb, vec3::Vec3};

use super::Pdf;

fn random_cosine_direction() -> Vec3 {
    let r1 = rand::thread_rng().gen_range(0.0f32..1.0);
    let r2 = rand::thread_rng().gen_range(0.0f32..1.0);
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * f32::consts::PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        Self {
            uvw: Onb::from_w(w),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> f32 {
        let cosine = direction.normalize().dot(&self.uvw.w());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / f32::consts::PI
        }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local(random_cosine_direction())
    }
}
