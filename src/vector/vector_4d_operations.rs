use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::vector::vector_4d::Vector4d;

impl Index<usize> for Vector4d where {
    type Output = f64;


    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Invalid vector dimension")
        }
    }
}

impl IndexMut<usize> for Vector4d {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Invalid vector dimension")
        }
    }
}

impl Mul<f64> for Vector4d {
    type Output = Vector4d;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w *rhs)
    }
}

impl Mul<Vector4d> for f64 {
    type Output = Vector4d;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        Vector4d::new(rhs.x * self, rhs.y * self, rhs.z * self, rhs.w * self)
    }
}

impl MulAssign<f64> for Vector4d {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl Div<f64> for Vector4d {
    type Output = Vector4d;

    fn div(self, rhs: f64) -> Self::Output {
        let multiplier = 1f64 / rhs;
        Vector4d::new(self.x * multiplier, self.y * multiplier, self.z * multiplier, self.w * multiplier)
    }
}

impl DivAssign<f64> for Vector4d {
    fn div_assign(&mut self, rhs: f64) {
        let multiplier = 1f64 / rhs;
        self.x *= multiplier;
        self.y *= multiplier;
        self.z *= multiplier;
        self.w *= multiplier;
    }
}

impl Neg for Vector4d {
    type Output = Vector4d;

    fn neg(self) -> Self::Output {
        self * -1.0f64
    }
}

impl Add for Vector4d {
    type Output = Vector4d;

    fn add(self, rhs: Self) -> Self::Output {
        Vector4d::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl AddAssign for Vector4d {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl Sub for Vector4d {
    type Output = Vector4d;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector4d::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl SubAssign for Vector4d {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}
