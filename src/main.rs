use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

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
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b: f64 = 0.25;

            let ir = (255.999 * r).trunc() as i32;
            let ig = (255.999 * g).trunc() as i32;
            let ib = (255.999 * b).trunc() as i32;

            writeln!(f, "{ir} {ig} {ib}").unwrap();
        }
    }
}
