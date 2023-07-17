use std::io::{self, Write};

extern crate rusty_rays;
use rusty_rays::camera::Camera;
use rusty_rays::color::{color_to_pixel, Color};
use rusty_rays::image::{Image, PPMImage};
use rusty_rays::material::{Lambertian, Metal, RayScatter};
use rusty_rays::objects::{Hittable, RayHit, Sphere, World};
use rusty_rays::ray::Ray;
use rusty_rays::utils::{random, INFINITY};
use rusty_rays::vec3::{unit_vector, Point3};

fn ray_color(r: &Ray, object: &impl Hittable, depth: u32) -> Color {
    if depth == 0 {
        return Color::zero();
    }
    let ray_hit = object.hit(r, 0.001, INFINITY);
    match ray_hit {
        RayHit::Hit(rec) => {
            match rec.mat.scatter(r, &rec) {
                RayScatter::Scatter(scattered) => {
                    scattered.attenuation * ray_color(&scattered.ray, object, depth - 1)
                }
                RayScatter::NoScatter => Color::zero(),
            }
            // let target = rec.p + rec.normal + Point3::random_unit_vector();
            // ray_color(&Ray::new(rec.p, target - rec.p), object, depth - 1) * 0.5
        }
        RayHit::NoHit => {
            // background
            let unit_direction = unit_vector(&r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
    // world
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);
    let mut world = World::new();
    world.add(&Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        &material_ground,
    ));
    world.add(&Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        &material_center,
    ));
    world.add(&Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        &material_left,
    ));
    world.add(&Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        &material_right,
    ));

    // camera
    let camera = Camera::new(16.0 / 9.0, 2.0, 1.0);

    // image
    let mut image = PPMImage::new("output.ppm", 400, 225);
    let samples_per_pixel = 100;
    let max_depth = 50;

    // render
    for j in (0..image.height()).rev() {
        print!("\rscanlines remaining: {j} ");
        io::stdout().flush().unwrap();
        for i in 0..image.width() {
            let mut pixel_color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random()) / (image.width() - 1) as f64;
                let v = (j as f64 + random()) / (image.height() - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            image.write(&color_to_pixel(&pixel_color, samples_per_pixel));
        }
    }

    println!("\nDone!");
}
