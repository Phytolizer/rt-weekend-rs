use crate::Color;
use crate::Point3;

pub mod checker;
pub mod image;
pub mod noise;
pub mod solid_color;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}
