use rand::{self, Rng};

extern crate rusty_rays;
use rusty_rays::camera::Camera;
use rusty_rays::color::{color_to_pixel, Color};
use rusty_rays::image::{Image, PPMImage};
use rusty_rays::objects::{Hittable, Sphere, World};
use rusty_rays::ray::Ray;
use rusty_rays::utils::INFINITY;
use rusty_rays::vec3::{unit_vector, Point3};

fn ray_color(r: &Ray, object: &impl Hittable) -> Color {
    let rec = object.hit(r, 0.0, INFINITY);
    if rec.hit {
        (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5
    } else {
        // background
        let unit_direction = unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    // world
    let mut world = World::new();
    world.add(&Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(&Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // camera
    let camera = Camera::new(16.0 / 9.0, 2.0, 1.0);

    // image
    let mut image = PPMImage::new("output.ppm", 400, 225);
    let samples_per_pixel = 100;

    // render
    let mut rng = rand::thread_rng();
    for j in (0..image.height()).rev() {
        println!("scanlines remaining: {j}");
        for i in 0..image.width() {
            let mut pixel_color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image.width() - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image.height() - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            image.write(&color_to_pixel(&pixel_color, samples_per_pixel));
        }
    }

    println!("Done!");
}
