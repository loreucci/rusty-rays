use serde::{Deserialize, Serialize};

use crate::ray::Ray;
use crate::utils::deg_to_rad;
use crate::vec3::{cross, unit_vector, Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

#[derive(Serialize, Deserialize)]
pub struct CameraDescription {
    lookfrom: [f64; 3],
    lookat: [f64; 3],
    vup: [f64; 3],
    vfov: f64,
    aspect_ratio: f64,
    aperture: f64,
    focus_dist: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        // viewport computation
        let theta = deg_to_rad(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        // position and orientation
        let w = unit_vector(&(lookfrom - lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);
        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;
        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn from(desc: &CameraDescription) -> Self {
        Self::new(
            Point3::new(desc.lookfrom[0], desc.lookfrom[1], desc.lookfrom[2]),
            Point3::new(desc.lookat[0], desc.lookat[1], desc.lookat[2]),
            Point3::new(desc.vup[0], desc.vup[1], desc.vup[2]),
            desc.vfov,
            desc.aspect_ratio,
            desc.aperture,
            desc.focus_dist,
        )
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disc() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
