use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub trait Image {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn write(&mut self, p: &Pixel);
}

pub struct PPMImage {
    w: u32,
    h: u32,
    f: BufWriter<File>,
}

impl PPMImage {
    pub fn new(file_name: &str, width: u32, height: u32) -> Self {
        let full_name = file_name.to_owned() + ".ppm";
        let mut f = BufWriter::new(File::create(full_name).expect("Unable to create file"));
        writeln!(f, "P3").unwrap();
        writeln!(f, "{width} {height}").unwrap();
        writeln!(f, "255").unwrap();
        Self {
            w: width,
            h: height,
            f,
        }
    }
}

impl Image for PPMImage {
    fn width(&self) -> u32 {
        self.w
    }

    fn height(&self) -> u32 {
        self.h
    }

    fn write(&mut self, p: &Pixel) {
        writeln!(self.f, "{} {} {}", p.r, p.g, p.b).unwrap();
    }
}
