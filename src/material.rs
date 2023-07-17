use crate::{
    color::Color,
    objects::HitRecord,
    ray::Ray,
    vec3::{dot, reflect, unit_vector, Vec3},
};

pub struct Scattered {
    pub attenuation: Color,
    pub ray: Ray,
}

pub enum RayScatter {
    Scatter(Scattered),
    NoScatter,
}

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> RayScatter;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> RayScatter {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal;
        }
        RayScatter::Scatter(Scattered {
            attenuation: self.albedo,
            ray: Ray::new(rec.p, scatter_direction),
        })
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> RayScatter {
        let reflected = reflect(&unit_vector(&r.direction()), &rec.normal);
        let dir = reflected + Vec3::random_in_unit_sphere() * self.fuzz;
        if dot(&dir, &rec.normal) > 0.0 {
            RayScatter::Scatter(Scattered {
                attenuation: self.albedo,
                ray: Ray::new(rec.p, dir),
            })
        } else {
            RayScatter::NoScatter
        }
    }
}
