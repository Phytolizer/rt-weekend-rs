use crate::perlin::Perlin;
use crate::Color;
use crate::Point3;

use super::Texture;

pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0
                + (self.scale * p.z + 10.0 * self.noise.turb(p, Perlin::DEFAULT_TURB_DEPTH)).sin())
    }
}
