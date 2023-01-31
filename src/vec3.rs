use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, Div, Index, Mul, Neg, Sub};

use rand::distributions::uniform::SampleRange;
use rand::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            i => panic!("Index {} out of bounds for Vec3!", i)
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Self::Output {
        Vec3 { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Self::Output {
        Vec3 { x: self.x / scalar, y: self.y / scalar, z: self.z / scalar }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn random<R>(rng: &mut ThreadRng, range: R) -> Self
        where R: SampleRange<f64> + std::clone::Clone
    {
        Vec3 {
            x: rng.gen_range(range.clone()),
            y: rng.gen_range(range.clone()),
            z: rng.gen_range(range),
        }
    }

    pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Vec3::random(rng, -1.0..1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_hemisphere(rng: &mut ThreadRng, normal: Vec3) -> Self {
        let in_unit_sphere = Vec3::random_in_unit_sphere(rng);
        // In same hemisphere as normal
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const V1: Vec3 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
    const V2: Vec3 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };

    #[test]
    fn test_neg_value() {
        assert_eq!(-V1, Vec3::new(-1.0, -2.0, -3.0))
    }

    #[test]
    fn test_add_value() {
        assert_eq!(V1 + V2, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_sub_value() {
        assert_eq!(V1 - V2, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_mul_val() {
        assert_eq!(V1 * 3.0, Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_div_val() {
        assert_eq!(V1 / 2.0, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", V1), "1 2 3\n");
    }
}