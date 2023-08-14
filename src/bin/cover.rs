extern crate rusty_rays;
use rusty_rays::camera::Camera;
use rusty_rays::color::Color;
use rusty_rays::image::PPMImage;
use rusty_rays::material::{Dielectric, Lambertian, Metal};
use rusty_rays::objects::{Sphere, World};
use rusty_rays::render::render;
use rusty_rays::utils::{random, random_between};
use rusty_rays::vec3::{Point3, Vec3};

fn main() {
    // cover world
    let mut world = World::new();

    // ground
    world.add(&Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        &Lambertian::new(Color::new(0.5, 0.5, 0.50)),
    ));

    // random small spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Point3::new(a as f64 + 0.9 * random(), 0.2, b as f64 + 0.9 * random());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }

            let m = if choose_mat < 0.8 {
                let albedo = Color::random() * Color::random();
                Lambertian::new(albedo)
            } else if choose_mat < 0.95 {
                let albedo = Color::random_between(0.5, 1.0);
                let fuzz = random_between(0.0, 0.5);
                Metal::new(albedo, fuzz)
            } else {
                Dielectric::new(1.5)
            };
            world.add(&Sphere::new(center, 0.2, &m));
        }
    }

    // big spheres
    world.add(&Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        &Dielectric::new(1.5),
    ));
    world.add(&Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        &Lambertian::new(Color::new(0.4, 0.2, 0.1)),
    ));
    world.add(&Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        &Metal::new(Color::new(0.7, 0.6, 0.5), 0.0),
    ));

    // camera (cover)
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        3.0 / 2.0,
        0.1,
        10.0,
    );

    // image
    let mut image = PPMImage::new("cover.ppm", 1280, 800);

    // render
    render(&world, &camera, &mut image, 100, 50);
}
