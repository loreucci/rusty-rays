use std::process;

use clap::Parser;

extern crate rusty_rays;
use rusty_rays::image::save_ppm;
use rusty_rays::render::render;
use rusty_rays::scene::parse_scene;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// output filename (without extension)
    #[arg(short, long, default_value_t = String::from("output"))]
    output: String,

    /// output width
    #[arg(short, long, default_value_t = 640)]
    width: u16,

    /// output height
    #[arg(short, long, default_value_t = 360)]
    height: u16,

    /// samples per pixel
    #[arg(short, long, default_value_t = 100)]
    samples: u32,

    /// max depth of rays
    #[arg(short, long, default_value_t = 50)]
    depth: u32,

    /// number of threads used for rendering
    #[arg(short, long, default_value_t = 1)]
    threads: u32,

    /// json file with the scene
    scene: String,
}

fn main() {
    // parse arguments
    let args = Args::parse();

    // world & camera
    let scene = parse_scene(&args.scene).unwrap_or_else(|err| {
        eprintln!("Unable to load scene from file '{}': {}", &args.scene, err);
        process::exit(1)
    });

    // render
    let image = render(
        &scene.world,
        &scene.camera,
        args.width,
        args.height,
        args.samples,
        args.depth,
        args.threads,
    );

    // save to file
    save_ppm(&args.output, &image).unwrap_or_else(|err| {
        eprintln!("Error saving file: {}", err);
        process::exit(1)
    });
}
