use crate::utils::{random, random_between};
use std::fmt::Display;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn zero() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }

    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn random() -> Self {
        Self {
            e: [random(), random(), random()],
        }
    }

    pub fn random_between(min: f64, max: f64) -> Self {
        Self {
            e: [
                random_between(min, max),
                random_between(min, max),
                random_between(min, max),
            ],
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_between(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        unit_vector(&Self::random_in_unit_sphere())
    }

    pub fn random_in_unit_disc() -> Self {
        loop {
            let p = Vec3::new(random_between(-1.0, 1.0), random_between(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        return self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2];
    }

    pub fn is_near_zero(&self) -> bool {
        let s = 1e-8;
        (self.e[0].abs() < s) && (self.e[1].abs() < s) && (self.e[2].abs() < s)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs)
    }
}

pub fn dot(x: &Vec3, y: &Vec3) -> f64 {
    return x.e[0] * y.e[0] + x.e[1] * y.e[1] + x.e[2] * y.e[2];
}

pub fn cross(x: &Vec3, y: &Vec3) -> Vec3 {
    Vec3::new(
        x.e[1] * y.e[2] - x.e[2] * y.e[1],
        x.e[2] * y.e[0] - x.e[0] * y.e[2],
        x.e[0] * y.e[1] - x.e[1] * y.e[0],
    )
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * 2.0 * dot(v, n)
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.e[0], self.e[1], self.e[2])
    }
}

pub type Point3 = Vec3;

#[cfg(test)]
mod tests {
    use super::Vec3;

    fn assert_is_close(x: f64, y: f64) {
        assert!((x - y).abs() < 0.0001);
    }

    #[test]
    fn vec_creation() {
        let v1 = Vec3::zero();
        assert_eq!(v1.e[0], 0.0);
        assert_eq!(v1.e[1], 0.0);
        assert_eq!(v1.e[2], 0.0);

        let v2 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v2.e[0], 1.0);
        assert_eq!(v2.e[1], 2.0);
        assert_eq!(v2.e[2], 3.0);
    }

    #[test]
    fn vec_neg() {
        let v = -Vec3::new(1.0, 2.0, 3.0);
        assert_is_close(v.e[0], -1.0);
        assert_is_close(v.e[1], -2.0);
        assert_is_close(v.e[2], -3.0);
    }

    #[test]
    fn vec_index() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_is_close(v1[0], 1.0);
        assert_is_close(v1[1], 2.0);
        assert_is_close(v1[2], 3.0);

        let mut v2 = Vec3::new(1.0, 2.0, 3.0);
        assert_is_close(v2[0], 1.0);
        assert_is_close(v2[1], 2.0);
        assert_is_close(v2[2], 3.0);
        v2[0] = 4.0;
        v2[1] = 5.0;
        v2[2] = 6.0;
        assert_is_close(v2[0], 4.0);
        assert_is_close(v2[1], 5.0);
        assert_is_close(v2[2], 6.0);
    }

    #[test]
    fn vec_assign_ops() {
        let mut v = Vec3::zero();

        v += Vec3::new(1.0, 2.0, 3.0);
        assert_is_close(v.e[0], 1.0);
        assert_is_close(v.e[1], 2.0);
        assert_is_close(v.e[2], 3.0);

        v -= Vec3::new(1.0, 1.0, 1.0);
        assert_is_close(v.e[0], 0.0);
        assert_is_close(v.e[1], 1.0);
        assert_is_close(v.e[2], 2.0);

        v *= 2.0;
        assert_is_close(v.e[0], 0.0);
        assert_is_close(v.e[1], 2.0);
        assert_is_close(v.e[2], 4.0);

        v /= 2.0;
        assert_is_close(v.e[0], 0.0);
        assert_is_close(v.e[1], 1.0);
        assert_is_close(v.e[2], 2.0);
    }

    #[test]
    fn vec_ops() {
        let v1 = Vec3::zero() + Vec3::new(1.0, 2.0, 3.0);
        assert_is_close(v1.e[0], 1.0);
        assert_is_close(v1.e[1], 2.0);
        assert_is_close(v1.e[2], 3.0);

        let v2 = v1 - Vec3::new(1.0, 1.0, 1.0);
        assert_is_close(v2.e[0], 0.0);
        assert_is_close(v2.e[1], 1.0);
        assert_is_close(v2.e[2], 2.0);

        let v3 = v2 * 2.0;
        assert_is_close(v3.e[0], 0.0);
        assert_is_close(v3.e[1], 2.0);
        assert_is_close(v3.e[2], 4.0);

        let v4 = v3 / 2.0;
        assert_is_close(v4.e[0], 0.0);
        assert_is_close(v4.e[1], 1.0);
        assert_is_close(v4.e[2], 2.0);
    }

    #[test]
    fn vec_length() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let exp = 1.0 + 4.0 + 9.0;
        assert_is_close(v.length_squared(), exp);
        assert_is_close(v.length(), exp.sqrt());
    }
}
