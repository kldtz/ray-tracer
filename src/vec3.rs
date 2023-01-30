use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, Div, Index, Mul, Neg, Sub};

const COL_FACTOR: f64 = 255.99;

#[derive(Debug, PartialEq)]
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

// TODO: use some kind of macro to cover all value-ref combinations

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl<'a> Neg for &'a Vec3 {
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

impl<'a> Add for &'a Vec3 {
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

impl<'a> Sub for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}


impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Self::Output {
        Vec3 { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar }
    }
}

impl<'a> Mul<f64> for &'a Vec3 {
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

impl<'a> Div<f64> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Self::Output {
        Vec3 { x: self.x / scalar, y: self.y / scalar, z: self.z / scalar }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
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

    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }

    pub fn to_color(&self) -> String {
        let color = self * COL_FACTOR;
        format!("{} {} {}", color.x as i64, color.y as i64, color.z as i64)
    }
}

pub type Point = Vec3;
pub type Color = Vec3;

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
    fn test_neg_ref() {
        assert_eq!(-&V1, Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_add_value() {
        assert_eq!(V1 + V2, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_add_ref() {
        assert_eq!(&V1 + &V1, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_sub_value() {
        assert_eq!(V1 - V2, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_sub_ref() {
        assert_eq!(&V1 - &V2, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_mul_val() {
        assert_eq!(V1 * 3.0, Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_mul_ref() {
        assert_eq!(&V1 * 3.0, Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_div_val() {
        assert_eq!(V1 / 2.0, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_div_ref() {
        assert_eq!(&V1 / 2.0, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", V1), "1 2 3\n");
    }
}