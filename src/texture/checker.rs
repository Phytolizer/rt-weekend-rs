use std::sync::Arc;

use crate::Color;
use crate::Point3;

use super::solid_color::SolidColor;
use super::Texture;

pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self { odd, even }
    }

    pub fn new_color(even: Color, odd: Color) -> Self {
        Self::new(
            Arc::new(SolidColor::new(even)),
            Arc::new(SolidColor::new(odd)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
