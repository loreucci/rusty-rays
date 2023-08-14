use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::camera::Camera;
use crate::color::{color_to_pixel, Color};
use crate::image::Image;
use crate::material::RayScatter;
use crate::objects::{Hittable, RayHit, World};
use crate::ray::Ray;
use crate::utils::{random, INFINITY};
use crate::vec3::unit_vector;

fn ray_color(r: &Ray, object: &impl Hittable, depth: u32) -> Color {
    if depth == 0 {
        return Color::zero();
    }
    let ray_hit = object.hit(r, 0.001, INFINITY);
    match ray_hit {
        RayHit::Hit(rec) => match rec.mat.scatter(r, &rec) {
            RayScatter::Scatter(scattered) => {
                scattered.attenuation * ray_color(&scattered.ray, object, depth - 1)
            }
            RayScatter::NoScatter => Color::zero(),
        },
        RayHit::NoHit => {
            // background
            let unit_direction = unit_vector(&r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    }
}

pub fn render(
    world: &World,
    camera: &Camera,
    width: u16,
    height: u16,
    samples_per_pixel: u32,
    max_depth: u32,
    threads: u32,
) -> Image {
    let _img = Image::new(width, height);
    let mut _img_it = _img.iter();
    let img = Arc::new(Mutex::new(_img));
    let img_it = Arc::new(Mutex::new(_img_it));
    let remaining = Arc::new(Mutex::new(height));

    thread::scope(|s| {
        for _ in 0..threads {
            let img = Arc::clone(&img);
            let img_it = Arc::clone(&img_it);
            let remaining = Arc::clone(&remaining);
            s.spawn(move || {
                loop {
                    let p = {
                        // let mut lock = img_it.lock().unwrap();
                        match img_it.lock().unwrap().next() {
                            Some(v) => v,
                            None => break,
                        }
                    };
                    {
                        let mut rem = remaining.lock().unwrap();
                        if *rem == p.y + 1 {
                            print!("\rscanlines remaining: {} ", *rem);
                            io::stdout().flush().unwrap();
                            *rem -= 1;
                        }
                    }
                    let mut pixel_color = Color::zero();
                    for _ in 0..samples_per_pixel {
                        let u = (p.x as f64 + random()) / (width - 1) as f64;
                        let v = (p.y as f64 + random()) / (height - 1) as f64;
                        let r = camera.get_ray(u, v);
                        pixel_color += ray_color(&r, world, max_depth);
                    }
                    {
                        img.lock()
                            .unwrap()
                            .set_color(&p, &color_to_pixel(&pixel_color, samples_per_pixel));
                    }
                }
            });
        }
    });

    println!("\nDone!");
    Arc::try_unwrap(img)
        .unwrap_or_else(|_| panic!("Error when accessing image mutex"))
        .into_inner()
        .unwrap()
}
