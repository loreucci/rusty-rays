use rand::{self, Rng};

pub const INFINITY: f64 = f64::INFINITY;

pub const PI: f64 = std::f64::consts::PI;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

static mut RNG: Option<rand::rngs::ThreadRng> = None;

pub fn random() -> f64 {
    unsafe {
        if RNG.is_none() {
            RNG = Some(rand::thread_rng());
        }
        RNG.as_mut().unwrap().gen::<f64>()
    }
}

pub fn random_between(min: f64, max: f64) -> f64 {
    unsafe {
        if RNG.is_none() {
            RNG = Some(rand::thread_rng());
        }
        RNG.as_mut().unwrap().gen::<f64>() * (max - min) + min
    }
}
