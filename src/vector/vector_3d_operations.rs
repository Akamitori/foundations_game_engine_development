﻿use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::vector::vector_3d::Vector3d;

impl Index<usize> for Vector3d where {
    type Output = f64;


    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid vector dimension")
        }
    }
}

impl IndexMut<usize> for Vector3d {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid vector dimension")
        }
    }
}

impl Mul<f64> for Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vector3d> for f64 {
    type Output = Vector3d;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        Vector3d::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl MulAssign<f64> for Vector3d {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vector3d {
    type Output = Vector3d;

    fn div(self, rhs: f64) -> Self::Output {
        let multiplier = 1f64 / rhs;
        Vector3d::new(self.x * multiplier, self.y * multiplier, self.z * multiplier)
    }
}

impl DivAssign<f64> for Vector3d {
    fn div_assign(&mut self, rhs: f64) {
        let multiplier = 1f64 / rhs;
        self.x *= multiplier;
        self.y *= multiplier;
        self.z *= multiplier;
    }
}

impl Neg for Vector3d {
    type Output = Vector3d;

    fn neg(self) -> Self::Output {
        self*-1.0f64
    }
}

impl Add for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3d::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}

impl AddAssign for Vector3d {
    fn add_assign(&mut self, rhs: Self) {
        self.x+=rhs.x;
        self.y+=rhs.x;
        self.z+=rhs.x;
    }
}

impl Sub for Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3d::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}

impl SubAssign for Vector3d {
    fn sub_assign(&mut self, rhs: Self) {
        self.x-=rhs.x;
        self.y-=rhs.x;
        self.z-=rhs.x;
    }
}
