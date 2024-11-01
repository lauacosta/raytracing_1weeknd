use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub},
    str::FromStr,
};

use crate::{random_f64, random_f64_range};

pub type Point3 = Vec3;

#[derive(Default, Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    #[must_use]
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
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

    #[inline]
    pub fn length_squared(&self) -> f64 {
        let e = self.e;
        e[0] * e[0] + e[1] * e[1] + e[2] * e[2]
    }

    #[inline]
    pub fn random() -> Self {
        Vec3::new(random_f64(), random_f64(), random_f64())
    }

    #[inline]
    pub fn random_with_range(min: f64, max: f64) -> Self {
        Vec3::new(
            random_f64_range(min, max),
            random_f64_range(min, max),
            random_f64_range(min, max),
        )
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;

        self.e.iter().all(|&component| component.abs() < s)
    }
}

#[inline]
pub fn dot(lhs: Vec3, rhs: Vec3) -> f64 {
    lhs.e[0] * rhs.e[0] + lhs.e[1] * rhs.e[1] + lhs.e[2] * rhs.e[2]
}
#[inline]
pub fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
    Vec3::new(
        lhs.e[1] * rhs.e[2] - lhs.e[2] * rhs.e[1],
        lhs.e[2] * rhs.e[0] - lhs.e[0] * rhs.e[2],
        lhs.e[0] * rhs.e[1] - lhs.e[1] * rhs.e[0],
    )
}

#[inline]
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

#[inline]
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random_f64_range(-1., 1.), random_f64_range(-1., 1.), 0.);

        if p.length_squared() < 1. {
            return p;
        }
    }
}

#[inline]
pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_with_range(-1., 1.);
        let lensq = p.length_squared();
        if (1e-160..=1.).contains(&lensq) {
            return p / lensq.sqrt();
        }
    }
}

#[inline]
pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(on_unit_sphere, *normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

#[inline]
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * dot(v, n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);

    let r_out_parallel = f64::abs(1.0 - r_out_perp.length_squared()).sqrt() * n;

    r_out_perp + r_out_parallel
}

impl MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl FromStr for Vec3 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();
        if coords.len() != 3 {
            return Err("Un vector tiene que estar en el formato 'x,y,z'".to_string());
        }

        let x = coords[0].parse::<f64>().map_err(|e| e.to_string())?;
        let y = coords[1].parse::<f64>().map_err(|e| e.to_string())?;
        let z = coords[2].parse::<f64>().map_err(|e| e.to_string())?;

        Ok(Vec3::new(x, y, z))
    }
}
