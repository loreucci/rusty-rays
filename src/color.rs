use crate::image::Pixel;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn color_to_pixel(c: &Color) -> Pixel {
    let r = (255.999 * c.x()).trunc() as u8;
    let g = (255.999 * c.y()).trunc() as u8;
    let b = (255.999 * c.z()).trunc() as u8;

    Pixel { r, g, b }
}
