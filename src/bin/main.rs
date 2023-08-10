use std::io::{self, Write};

extern crate rusty_rays;
use rusty_rays::camera::Camera;
use rusty_rays::color::{color_to_pixel, Color};
use rusty_rays::image::{Image, PPMImage};
use rusty_rays::material::{Dielectric, Lambertian, Metal, RayScatter};
use rusty_rays::objects::{Hittable, RayHit, Sphere, World};
use rusty_rays::ray::Ray;
use rusty_rays::utils::{random, random_between, INFINITY};
use rusty_rays::vec3::{unit_vector, Point3, Vec3};

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

fn main() {
    // // world
    // let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    // let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    // let material_left = Dielectric::new(1.5);
    // let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);
    // let mut world = World::new();
    // world.add(&Sphere::new(
    //     Point3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     &lambertians[0],
    // ));
    // world.add(&Sphere::new(
    //     Point3::new(0.0, 0.0, -1.0),
    //     0.5,
    //     &material_center,
    // ));
    // world.add(&Sphere::new(
    //     Point3::new(-1.0, 0.0, -1.0),
    //     0.5,
    //     &material_left,
    // ));
    // world.add(&Sphere::new(
    //     Point3::new(-1.0, 0.0, -1.0),
    //     -0.4,
    //     &material_left,
    // ));
    // world.add(&Sphere::new(
    //     Point3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     &material_right,
    // ));

    // cover world
    // materials
    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.50));
    let mut lambertians: Vec<Lambertian> = vec![];
    let mut lambertians_c: Vec<Point3> = vec![];
    let mut metals: Vec<Metal> = vec![];
    let mut metals_c: Vec<Point3> = vec![];
    let mut dielectrics: Vec<Dielectric> = vec![];
    let mut dielectrics_c: Vec<Point3> = vec![];
    let material1 = Dielectric::new(1.5);
    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);

    let mut world = World::new();

    // ground
    world.add(&Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        &ground_material,
    ));

    // random small spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Point3::new(a as f64 + 0.9 * random(), 0.2, b as f64 + 0.9 * random());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }

            if choose_mat < 0.8 {
                let albedo = Color::random() * Color::random();
                lambertians.push(Lambertian::new(albedo));
                lambertians_c.push(center);
            } else if choose_mat < 0.95 {
                let albedo = Color::random_between(0.5, 1.0);
                let fuzz = random_between(0.0, 0.5);
                metals.push(Metal::new(albedo, fuzz));
                metals_c.push(center);
            } else {
                dielectrics.push(Dielectric::new(1.5));
                dielectrics_c.push(center);
            }
        }
    }
    for i in 0..lambertians.len() {
        world.add(&Sphere::new(lambertians_c[i], 0.2, &lambertians[i]));
    }
    for i in 0..metals.len() {
        world.add(&Sphere::new(metals_c[i], 0.2, &metals[i]));
    }
    for i in 0..dielectrics.len() {
        world.add(&Sphere::new(dielectrics_c[i], 0.2, &dielectrics[i]));
    }

    // big spheres
    world.add(&Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, &material1));
    world.add(&Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, &material2));
    world.add(&Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, &material3));

    // // camera
    // let camera = Camera::new(
    //     Point3::new(-2.0, 2.0, 1.0),
    //     Point3::new(0.0, 0.0, -1.0),
    //     Vec3::new(0.0, 1.0, 0.0),
    //     90.0,
    //     16.0 / 9.0,
    // );

    // // camera (zoomed)
    // let lookfrom = Point3::new(3.0, 3.0, 2.0);
    // let lookat = Point3::new(0.0, 0.0, -1.0);
    // let camera = Camera::new(
    //     lookfrom,
    //     lookat,
    //     Vec3::new(0.0, 1.0, 0.0),
    //     20.0,
    //     16.0 / 9.0,
    //     2.0,
    //     (lookfrom - lookat).length(),
    // );

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
    let mut image = PPMImage::new("output.ppm", 1200, 800);
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
