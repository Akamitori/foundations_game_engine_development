use std::ops::Mul;

use crate::quaternion::quaternion::Quaternion;
use crate::transform::transform_vector::{TransformVector, VectorWrapper};
use crate::vector::vector_3d_operations::{cross, dot};

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        let q1 = self;
        let q2 = rhs;
        Quaternion::new(
            q1.w * q2.x + q1.x * q2.w + q1.y * q2.z - q1.z * q2.y,
            q1.w * q2.y - q1.x * q2.z + q1.y * q2.w + q1.z * q2.x,
            q1.w * q2.z + q1.x * q2.y - q1.y * q2.x + q1.z * q2.w,
            q1.w * q2.w - q1.x * q2.x - q1.y * q2.y - q1.z * q2.z,
        )
    }
}

impl Mul<Quaternion> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        let q1 = self;
        let q2 = rhs;
        Quaternion::new(
            q1.w * q2.x + q1.x * q2.w + q1.y * q2.z - q1.z * q2.y,
            q1.w * q2.y - q1.x * q2.z + q1.y * q2.w + q1.z * q2.x,
            q1.w * q2.z + q1.x * q2.y - q1.y * q2.x + q1.z * q2.w,
            q1.w * q2.w - q1.x * q2.x - q1.y * q2.y - q1.z * q2.z,
        )
    }
}

impl Mul<&Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: &Quaternion) -> Self::Output {
        let q1 = self;
        let q2 = rhs;
        Quaternion::new(
            q1.w * q2.x + q1.x * q2.w + q1.y * q2.z - q1.z * q2.y,
            q1.w * q2.y - q1.x * q2.z + q1.y * q2.w + q1.z * q2.x,
            q1.w * q2.z + q1.x * q2.y - q1.y * q2.x + q1.z * q2.w,
            q1.w * q2.w - q1.x * q2.x - q1.y * q2.y - q1.z * q2.z,
        )
    }
}

impl Mul<&Quaternion> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: &Quaternion) -> Self::Output {
        let q1 = self;
        let q2 = rhs;
        Quaternion::new(
            q1.w * q2.x + q1.x * q2.w + q1.y * q2.z - q1.z * q2.y,
            q1.w * q2.y - q1.x * q2.z + q1.y * q2.w + q1.z * q2.x,
            q1.w * q2.z + q1.x * q2.y - q1.y * q2.x + q1.z * q2.w,
            q1.w * q2.w - q1.x * q2.x - q1.y * q2.y - q1.z * q2.z,
        )
    }
}

impl<T: VectorWrapper> AsRef<TransformVector<T>> for Quaternion {
    fn as_ref(&self) -> &TransformVector<T> {
        unsafe { &*(self as *const Quaternion as *const TransformVector<T>) }
    }
}

impl<T: VectorWrapper> AsMut<TransformVector<T>> for Quaternion {
    fn as_mut(&mut self) -> &mut TransformVector<T> {
        unsafe { &mut *(self as *mut Quaternion as *mut TransformVector<T>) }
    }
}

pub fn transform<T: VectorWrapper>(v: &TransformVector<T>, q: &Quaternion) -> TransformVector<T> {
    let v = v.get_vector();
    let b = q.get_vector_part::<T>().get_vector();
    let b2 = b.x * b.x + b.y * b.y + b.z * b.z;

    let a = v * (q.w * q.w - b2) + b * (dot(v, b) * 2f64) + cross(b, v) * (q.w * 2.0f64);
    TransformVector::<T>::from(a)
}

