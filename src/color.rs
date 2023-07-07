use crate::vec3::Vec3;
use std::fs::File;
use std::io::BufWriter;
use std::io::{Error, Write};

pub type Color = Vec3;

pub fn write_color(f: &mut BufWriter<File>, c: &Color) -> Result<(), Error> {
    let r = (255.999 * c.x()).trunc() as i32;
    let g = (255.999 * c.y()).trunc() as i32;
    let b = (255.999 * c.z()).trunc() as i32;

    writeln!(f, "{r} {g} {b}")
}
