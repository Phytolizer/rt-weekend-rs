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

    pub fn new_rgb(red: f32, green: f32, blue: f32) -> Self {
        Self {
            color_value: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _p: Point3) -> Color {
        self.color_value
    }
}
