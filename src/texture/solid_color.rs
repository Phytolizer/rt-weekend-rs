use crate::Color;
use crate::Point3;

use super::Texture;

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color_value: Color) -> Self {
        Self { color_value }
    }

    pub fn new_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color_value: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.color_value
    }
}
