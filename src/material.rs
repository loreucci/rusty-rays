use crate::{
    color::Color,
    objects::HitRecord,
    ray::Ray,
    utils::random,
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

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> RayScatter {
        let refraction_ratio = if rec.front_face {
            (1.0) / self.ir
        } else {
            self.ir
        };
        let unit_direction = unit_vector(&r.direction());
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let refractance = {
            let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);
            r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
        };
        let direction = if refraction_ratio * sin_theta > 1.0 || refractance > random() {
            reflect(&unit_direction, &rec.normal)
        } else {
            let uv = &unit_direction;
            let n = &rec.normal;
            let r_out_perp = (*uv + *n * cos_theta) * refraction_ratio;
            let r_out_parallel = -*n * (1.0 - r_out_perp.length_squared()).sqrt();
            r_out_perp + r_out_parallel
        };

        RayScatter::Scatter(Scattered {
            attenuation: Color::new(1.0, 1.0, 1.0),
            ray: Ray::new(rec.p, direction),
        })
    }
}
