use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub enum RayHit {
    Hit(HitRecord),
    NoHit,
}

impl HitRecord {
    pub fn new(p: &Point3, t: f64, r: &Ray, outward_normal: &Vec3) -> Self {
        let front_face = dot(&r.direction(), outward_normal) < 0.0;
        Self {
            p: *p,
            normal: if front_face {
                *outward_normal
            } else {
                -*outward_normal
            },
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> RayHit;
}

#[derive(Copy, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
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
        RayHit::Hit(HitRecord::new(&intersection, root, r, &outward_normal))
    }
}

pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add<T: Hittable + Copy + 'static>(&mut self, obj: &T) {
        self.objects.push(Box::new(*obj));
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
            RayHit::Hit(rec) => rec.t,
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

#[cfg(test)]
mod tests {
    use super::{Point3, Sphere, World};

    #[test]
    fn world() {
        let mut w = World::new();
        w.add(&Sphere::new(Point3::zero(), 0.0));
    }
}
