use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::material::MaterialPtr;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: MaterialPtr,
    pub t: f64,
    pub front_face: bool,
}

pub enum RayHit {
    Hit(HitRecord),
    NoHit,
}

impl HitRecord {
    pub fn new(p: &Point3, t: f64, r: &Ray, outward_normal: &Vec3, mat: &MaterialPtr) -> Self {
        let front_face = dot(&r.direction(), outward_normal) < 0.0;
        Self {
            p: *p,
            normal: if front_face {
                *outward_normal
            } else {
                -*outward_normal
            },
            mat: mat.clone(),
            t,
            front_face,
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> RayHit;
}

pub type Object = Arc<dyn Hittable>;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: MaterialPtr,
}

#[derive(Serialize, Deserialize)]
pub struct SphereDescription {
    center: [f64; 3],
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: &MaterialPtr) -> Object {
        Arc::new(Self {
            center,
            radius,
            mat: mat.clone(),
        })
    }

    pub fn from(desc: &SphereDescription, mat: &MaterialPtr) -> Object {
        Self::new(
            Point3::new(desc.center[0], desc.center[1], desc.center[2]),
            desc.radius,
            mat,
        )
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> RayHit {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return RayHit::NoHit;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return RayHit::NoHit;
            }
        }
        let intersection = r.at(root);
        let outward_normal = (intersection - self.center) / self.radius;
        RayHit::Hit(HitRecord::new(
            &intersection,
            root,
            r,
            &outward_normal,
            &self.mat,
        ))
    }
}

pub struct World {
    objects: Vec<Object>,
}

impl World {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, obj: &Object) {
        self.objects.push(obj.clone());
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    fn hit_recursive(&self, r: &Ray, t_min: f64, t_max: f64, i: usize) -> RayHit {
        if i >= self.objects.len() {
            return RayHit::NoHit;
        }
        let obj = &self.objects[i];
        let ray_hit = obj.hit(r, t_min, t_max);
        let closest_so_far = match ray_hit {
            RayHit::NoHit => t_max,
            RayHit::Hit(ref rec) => rec.t,
        };
        let ray_hit_next = self.hit_recursive(r, t_min, closest_so_far, i + 1);
        match ray_hit_next {
            RayHit::NoHit => ray_hit,
            RayHit::Hit(_) => ray_hit_next,
        }
    }
}

impl Hittable for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> RayHit {
        self.hit_recursive(r, t_min, t_max, 0)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ObjectDescription {
    #[serde(rename = "sphere")]
    Sphere(SphereDescription),
}

pub fn create_object(desc: &ObjectDescription, mat: &MaterialPtr) -> Object {
    match desc {
        ObjectDescription::Sphere(d) => Sphere::from(d, mat),
    }
}
