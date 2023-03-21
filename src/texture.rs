use crate::Color;
use crate::Point3;

pub mod solid_color;
pub mod checker;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}
