use std::io::BufRead;
use std::io::Seek;

use image::RgbImage;

use crate::Color;
use crate::Point3;

use super::Texture;

pub struct ImageTexture {
    data: RgbImage,
    width: u32,
    height: u32,
}

impl ImageTexture {
    pub fn new<R: BufRead + Seek>(reader: &mut R) -> Self {
        let data = image::io::Reader::new(reader)
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
            .into_rgb8();
        Self {
            width: data.width(),
            height: data.height(),
            data,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * self.width as f64) as u32;
        let j = (v * self.height as f64) as u32;

        let i = i.min(self.width - 1);
        let j = j.min(self.height - 1);

        const color_scale: f64 = 1.0 / 255.0;
        let pixel = self.data.get_pixel(i, j);

        Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}
