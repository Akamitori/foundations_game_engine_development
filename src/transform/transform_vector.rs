use std::fmt::{Debug, Display, Formatter};

use crate::transform::transform_vector_operations::dot;
use crate::vector::Vector3d;
use crate::vector::vector_3d_operations::dot as innerDot;

pub trait VectorWrapper : Debug
{
    fn new(x: f64, y: f64, z: f64) -> Self;
    fn get_vector(&self) -> &Vector3d;
    fn get_vector_mut(&mut self) -> &mut Vector3d;
}



#[derive(Default, Clone)]
#[repr(transparent)]
pub struct TransformVector<T: VectorWrapper>(T);

impl<T: VectorWrapper> TransformVector<T> {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        TransformVector::<T> { 0: T::new(x, y, z) }
    }

    pub fn get_vector(&self) -> &Vector3d {
        self.0.get_vector()
    }

    pub fn get_vector_mut(&mut self) -> &mut Vector3d {
        self.0.get_vector_mut()
    }

    #[inline(always)]
    pub fn magnitude(&self) -> f64 {
        let self_vec = self.get_vector();
        (self_vec.x * self_vec.x + self_vec.y * self_vec.y + self_vec.z * self_vec.z).sqrt()
    }

    #[inline(always)]
    pub fn normalize(&self) -> Self {
        return self / self.magnitude();
    }

    #[inline(always)]
    pub fn project<U: VectorWrapper>(&self, to: &TransformVector<U>) -> Self {
        let result = to * dot(self, to) / dot(to, to);
        let result_vec = result.get_vector();
        Self::new(result_vec.x, result_vec.y, result_vec.z)
    }

    #[inline(always)]
    pub fn reject<U: VectorWrapper>(&self, from: &TransformVector<U>) -> Self {
        let self_vec = self.get_vector();
        let from_vec = from.get_vector();
        let result =
            self_vec - from_vec * innerDot(self_vec, from_vec) / innerDot(from_vec, from_vec);
        Self::new(result.x, result.y, result.z)
    }
}

impl<T: VectorWrapper> Debug for TransformVector<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",&self.0)
    }
}


impl<T: VectorWrapper> From<Vector3d> for TransformVector<T> {
    fn from(value: Vector3d) -> Self {
        TransformVector::<T> {
            0: T::new(value.x, value.y, value.z),
        }
    }
}
