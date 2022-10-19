use std::ops::{Add, Index, Mul, Neg, Sub};

use image::Rgb;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T>> Mul for Vec3<T> {
    type Output = T;

    fn mul(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

pub trait Cross {
    type Output;

    fn cross(&self, other: Self) -> Self::Output;
}

impl<T: Copy + Mul<Output = T> + Sub<Output = T>> Cross for Vec3<T> {
    type Output = Self;

    fn cross(&self, other: Self) -> Self {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        if index == 0 {
            &self.x
        } else if index == 1 {
            &self.y
        } else if index == 2 {
            &self.z
        } else {
            panic!("Out of bounds indexing Vec3");
        }
    }
}

pub trait Norm {
    type Output: ?Sized;

    fn norm(&self) -> Self::Output;
}

impl Norm for Vec3<f32> {
    type Output = f32;

    fn norm(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }
}

impl Norm for Vec3<f64> {
    type Output = f64;

    fn norm(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }
}

impl Norm for Vec3<i32> {
    type Output = f64;

    fn norm(&self) -> f64 {
        f64::sqrt(f64::from(
            self.x * self.x + self.y * self.y + self.z * self.z,
        ))
    }
}

pub trait Normalize {
    type Output;

    fn normalize(&self) -> Self::Output;
}

impl Normalize for Vec3<f32> {
    type Output = Self;

    fn normalize(&self) -> Self {
        *self * (1. / self.norm())
    }
}

impl Normalize for Vec3<f64> {
    type Output = Self;

    fn normalize(&self) -> Self {
        *self * (1. / self.norm())
    }
}

impl From<Vec3<f64>> for Rgb<u8> {
    fn from(orig: Vec3<f64>) -> Self {
        Rgb::from([
            (orig.x.min(1.) * 255.) as u8,
            (orig.y.min(1.) * 255.) as u8,
            (orig.z.min(1.) * 255.) as u8,
        ])
    }
}
