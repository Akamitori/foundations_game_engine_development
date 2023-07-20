use std::ops::{Add, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg};

use crate::transform::transform_vector::{TransformVector, VectorWrapper};
use crate::transform::Vector3D;
use crate::vector::vector_3d_operations::{cross as innerCross, dot as innerDot};

impl<T: VectorWrapper> Index<usize> for TransformVector<T> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        let vec = self.get_vector();
        match index {
            0 => &vec.x,
            1 => &vec.y,
            2 => &vec.z,
            _ => panic!("Invalid vector dimension"),
        }
    }
}

impl<T: VectorWrapper> IndexMut<usize> for TransformVector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let vec = self.get_vector_mut();
        match index {
            0 => &mut vec.x,
            1 => &mut vec.y,
            2 => &mut vec.z,
            _ => panic!("Invalid vector dimension"),
        }
    }
}

impl<T: VectorWrapper> Mul<f64> for TransformVector<T> {
    type Output = TransformVector<T>;

    fn mul(self, rhs: f64) -> Self::Output {
        let self_vec = self.get_vector();
        let result = self_vec * rhs;
        TransformVector::from(result)
    }
}

impl<T: VectorWrapper> Mul<f64> for &TransformVector<T> {
    type Output = TransformVector<T>;

    fn mul(self, rhs: f64) -> Self::Output {
        let self_vec = self.get_vector();
        let result = self_vec * rhs;
        TransformVector::from(result)
    }
}

impl<T: VectorWrapper> Mul<TransformVector<T>> for f64 {
    type Output = TransformVector<T>;

    fn mul(self, rhs: TransformVector<T>) -> Self::Output {
        let rhs = rhs.get_vector();
        let result = self * rhs;
        TransformVector::from(result)
    }
}

impl<T: VectorWrapper> Mul<&TransformVector<T>> for f64 {
    type Output = TransformVector<T>;

    fn mul(self, rhs: &TransformVector<T>) -> Self::Output {
        let rhs = rhs.get_vector();
        let result = self * rhs;
        TransformVector::from(result)
    }
}

impl<T: VectorWrapper> MulAssign<f64> for TransformVector<T> {
    fn mul_assign(&mut self, rhs: f64) {
        let vec = self.get_vector_mut();
        vec.x *= rhs;
        vec.y *= rhs;
        vec.z *= rhs;
    }
}

impl<T: VectorWrapper> Div<f64> for TransformVector<T> {
    type Output = TransformVector<T>;

    fn div(self, rhs: f64) -> Self::Output {
        let result = self.get_vector() / rhs;
        TransformVector::from(result)
    }
}

impl<T: VectorWrapper> Div<f64> for &TransformVector<T> {
    type Output = TransformVector<T>;

    fn div(self, rhs: f64) -> Self::Output {
        let result = self.get_vector() / rhs;
        TransformVector::from(result)
    }
}

impl<T: VectorWrapper> DivAssign<f64> for TransformVector<T> {
    fn div_assign(&mut self, rhs: f64) {
        let multiplier = 1f64 / rhs;
        let vec = self.get_vector_mut();
        vec.x *= multiplier;
        vec.y *= multiplier;
        vec.z *= multiplier;
    }
}

impl<T: VectorWrapper> Neg for TransformVector<T> {
    type Output = TransformVector<T>;

    fn neg(self) -> Self::Output {
        self * -1.0f64
    }
}

impl<T: VectorWrapper> Add<TransformVector<T>> for TransformVector<T> {
    type Output = TransformVector<T>;

    fn add(self, rhs: TransformVector<T>) -> Self::Output {
        let self_vec = self.get_vector();
        let rhs_vec = rhs.get_vector();

        TransformVector::<T>::new(
            self_vec.x + rhs_vec.x,
            self_vec.y + rhs_vec.y,
            self_vec.z + rhs_vec.z,
        )
    }
}

impl<T: VectorWrapper> Add<&TransformVector<T>> for TransformVector<T> {
    type Output = TransformVector<T>;

    fn add(self, rhs: &TransformVector<T>) -> Self::Output {
        let self_vec = self.get_vector();
        let rhs_vec = rhs.get_vector();

        TransformVector::<T>::new(
            self_vec.x + rhs_vec.x,
            self_vec.y + rhs_vec.y,
            self_vec.z + rhs_vec.z,
        )
    }
}

impl<T: VectorWrapper> Add<TransformVector<T>> for &TransformVector<T> {
    type Output = TransformVector<T>;

    fn add(self, rhs: TransformVector<T>) -> Self::Output {
        let self_vec = self.get_vector();
        let rhs_vec = rhs.get_vector();

        TransformVector::<T>::new(
            self_vec.x + rhs_vec.x,
            self_vec.y + rhs_vec.y,
            self_vec.z + rhs_vec.z,
        )
    }
}

impl<T: VectorWrapper> Add<&TransformVector<T>> for &TransformVector<T> {
    type Output = TransformVector<T>;

    fn add(self, rhs: &TransformVector<T>) -> Self::Output {
        let self_vec = self.get_vector();
        let rhs_vec = rhs.get_vector();

        TransformVector::<T>::new(
            self_vec.x + rhs_vec.x,
            self_vec.y + rhs_vec.y,
            self_vec.z + rhs_vec.z,
        )
    }
}

#[inline(always)]
pub fn dot<T: VectorWrapper, U: VectorWrapper>(
    a: &TransformVector<T>,
    b: &TransformVector<U>,
) -> f64 {
    let a = a.get_vector();
    let b = b.get_vector();
    return innerDot(a, b);
}

#[inline(always)]
pub fn cross<T: VectorWrapper, U: VectorWrapper>(
    a: &TransformVector<T>,
    b: &TransformVector<U>,
) -> Vector3D {
    let a = a.get_vector();
    let b = b.get_vector();
    return Vector3D::from(innerCross(a, b));
}
