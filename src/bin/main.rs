extern crate rusty_rays;
use rusty_rays::color::{color_to_pixel, Color};
use rusty_rays::image::{Image, PPMImage};
use rusty_rays::objects::{Hittable, Sphere, World};
use rusty_rays::ray::Ray;
use rusty_rays::utils::INFINITY;
use rusty_rays::vec3::{unit_vector, Point3, Vec3};

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
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // image
    let mut image = PPMImage::new("output.ppm", 400, 225);

    // render
    for j in (0..image.height()).rev() {
        println!("scanlines remaining: {j}");
        for i in 0..image.width() {
            let u = i as f64 / (image.width() - 1) as f64;
            let v = j as f64 / (image.height() - 1) as f64;
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let pixel_color = ray_color(&r, &world);
            image.write(&color_to_pixel(&pixel_color));
        }
    }

    println!("Done!");
}
