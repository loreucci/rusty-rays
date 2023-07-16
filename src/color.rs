use crate::image::Pixel;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn color_to_pixel(c: &Color, samples_per_pixel: i32) -> Pixel {
    let scale = 1.0 / samples_per_pixel as f64;

    Pixel {
        r: (256.0 * (c.x() * scale).clamp(0.0, 0.999)) as u8,
        g: (256.0 * (c.y() * scale).clamp(0.0, 0.999)) as u8,
        b: (256.0 * (c.z() * scale).clamp(0.0, 0.999)) as u8,
    }
}
