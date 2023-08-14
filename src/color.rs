use crate::image::PixelColor;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn color_to_pixel(c: &Color, samples_per_pixel: u32) -> PixelColor {
    let scale = 1.0 / samples_per_pixel as f64;

    PixelColor {
        r: (256.0 * (c.x() * scale).sqrt().clamp(0.0, 0.999)) as u8,
        g: (256.0 * (c.y() * scale).sqrt().clamp(0.0, 0.999)) as u8,
        b: (256.0 * (c.z() * scale).sqrt().clamp(0.0, 0.999)) as u8,
    }
}
