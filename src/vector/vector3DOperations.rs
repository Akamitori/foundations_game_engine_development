use std::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::vector::vector3D::Vector3D;

impl Index<usize> for Vector3D where {
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

impl IndexMut<usize> for Vector3D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid vector dimension")
        }
    }
}

impl Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vector3D> for f64 {
    type Output = Vector3D;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        Vector3D::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl MulAssign<f64> for Vector3D {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f64) -> Self::Output {
        let multiplier = 1f64 / rhs;
        Vector3D::new(self.x * multiplier, self.y * multiplier, self.z * multiplier)
    }
}

impl DivAssign<f64> for Vector3D {
    fn div_assign(&mut self, rhs: f64) {
        let multiplier = 1f64 / rhs;
        self.x *= multiplier;
        self.y *= multiplier;
        self.z *= multiplier;
    }
}

impl Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Self::Output {
        self*-1.0f64
    }
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3D::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}

impl AddAssign for Vector3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x+=rhs.x;
        self.y+=rhs.x;
        self.z+=rhs.x;
    }
}

impl Sub for Vector3D{
    type Output = Vector3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3D::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}

impl SubAssign for Vector3D{
    fn sub_assign(&mut self, rhs: Self) {
        self.x-=rhs.x;
        self.y-=rhs.x;
        self.z-=rhs.x;
    }
}

