use std::fs::File;
use std::io::{self, BufWriter, Write};

pub struct PixelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct PixelCoord {
    pub x: u16,
    pub y: u16,
}

pub struct Image {
    data: Vec<u8>,
    width: u16,
    height: u16,
}

impl Image {
    pub fn new(width: u16, height: u16) -> Self {
        let size = width as usize * height as usize * 3;
        Self {
            data: vec![0; size],
            width,
            height,
        }
    }

    pub fn set_color(&mut self, p: &PixelCoord, c: &PixelColor) {
        let i = 3 * (p.y as usize * self.width as usize + p.x as usize);
        self.data[i] = c.r;
        self.data[i + 1] = c.g;
        self.data[i + 2] = c.b;
    }

    pub fn get_color(&self, p: &PixelCoord) -> PixelColor {
        let i = 3 * (p.y as usize * self.width as usize + p.x as usize);
        PixelColor {
            r: self.data[i],
            g: self.data[i + 1],
            b: self.data[i + 2],
        }
    }

    pub fn iter(&self) -> ImageIterator {
        ImageIterator {
            x: 0,
            y: self.height - 1,
            width: self.width,
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }
}

pub struct ImageIterator {
    x: u16,
    y: u16,
    width: u16,
}

impl Iterator for ImageIterator {
    type Item = PixelCoord;
    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.width && self.y == 0 {
            return None;
        }
        if self.x >= self.width {
            self.y -= 1;
            self.x = 0;
        }
        let ret = PixelCoord {
            x: self.x,
            y: self.y,
        };
        self.x += 1;
        Some(ret)
    }
}

pub fn save_ppm(file_name: &str, image: &Image) -> Result<(), String> {
    print!("Saving file... ");
    io::stdout().flush().unwrap();
    let full_name = file_name.to_owned() + ".ppm";
    let file = match File::create(full_name) {
        io::Result::Ok(v) => v,
        io::Result::Err(e) => return Err(e.to_string()),
    };
    let mut buf = BufWriter::new(file);
    writeln!(buf, "P3").unwrap();
    writeln!(buf, "{} {}", image.width, image.height).unwrap();
    writeln!(buf, "255").unwrap();
    for p in image.iter() {
        let c = image.get_color(&p);
        writeln!(buf, "{} {} {}", c.r, c.g, c.b).unwrap();
    }
    println!("done!");
    Ok(())
}
