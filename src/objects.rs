use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

pub struct HitRecord {
    pub hit: bool,
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn no_hit() -> Self {
        Self {
            hit: false,
            p: Point3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn hit(p: &Point3, t: f64, r: &Ray, outward_normal: &Vec3) -> Self {
        let front_face = dot(&r.direction(), outward_normal) < 0.0;
        Self {
            hit: true,
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> HitRecord;
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> HitRecord {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return HitRecord::no_hit();
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return HitRecord::no_hit();
            }
        }
        let intersection = r.at(root);
        let outward_normal = (intersection - self.center) / self.radius;
        HitRecord::hit(&intersection, root, r, &outward_normal)
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
}

impl Hittable for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> HitRecord {
        let mut hit_record = HitRecord::no_hit();
        let mut closest_so_far = t_max;
        for object in &self.objects {
            let rec = object.hit(r, t_min, closest_so_far);
            if rec.hit {
                hit_record = rec;
                closest_so_far = hit_record.t;
            }
        }
        hit_record
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
