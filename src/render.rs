use std::io::{self, Write};

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
    image: &mut dyn Image,
    samples_per_pixel: u32,
    max_depth: u32,
) {
    for j in (0..image.height()).rev() {
        print!("\rscanlines remaining: {j} ");
        io::stdout().flush().unwrap();
        for i in 0..image.width() {
            let mut pixel_color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random()) / (image.width() - 1) as f64;
                let v = (j as f64 + random()) / (image.height() - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, world, max_depth);
            }
            image.write(&color_to_pixel(&pixel_color, samples_per_pixel));
        }
    }

    println!("\nDone!");
}
