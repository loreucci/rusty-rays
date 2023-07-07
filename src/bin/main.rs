use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

extern crate rusty_rays;
use rusty_rays::color::{write_color, Color};

fn main() {
    // image
    let image_width = 256;
    let image_height = 256;

    // file
    let mut f = BufWriter::new(File::create("output.ppm").expect("Unable to create file"));

    // render
    writeln!(f, "P3").unwrap();
    writeln!(f, "{image_width} {image_height}").unwrap();
    writeln!(f, "255").unwrap();

    for j in (0..image_height).rev() {
        println!("scanlines remaining: {j}");
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );

            write_color(&mut f, &pixel_color).unwrap();
        }
    }

    println!("Done!");
}
