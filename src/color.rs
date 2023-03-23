use image::RgbImage;

use crate::Color;

pub fn write_color(
    out: &mut RgbImage,
    pos: (usize, usize),
    pixel_color: &Color,
    samples_per_pixel: usize,
) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    if r.is_nan() {
        r = 0.0;
    }
    if g.is_nan() {
        g = 0.0;
    }
    if b.is_nan() {
        b = 0.0;
    }

    let scale = 1.0 / samples_per_pixel as f32;

    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    out.put_pixel(
        pos.0 as u32,
        pos.1 as u32,
        image::Rgb([
            (256.0 * r.clamp(0.0, 0.999)) as u8,
            (256.0 * g.clamp(0.0, 0.999)) as u8,
            (256.0 * b.clamp(0.0, 0.999)) as u8,
        ]),
    );
}
