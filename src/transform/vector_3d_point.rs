use std::fmt::{Display, Formatter, write};
use crate::transform::transform_vector::{TransformVector, VectorWrapper};
use crate::vector::Vector3d;
use std::ops::{Deref, DerefMut};

pub type Point3D = TransformVector<Vector3dPoint>;

#[derive(Default, Debug, Clone)]
#[repr(transparent)]
pub struct Vector3dPoint {
    v: Vector3d,
}

impl VectorWrapper for Vector3dPoint {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3dPoint {
            v: Vector3d::new(x, y, z),
        }
    }

    fn get_vector(&self) -> &Vector3d {
        &self.v
    }

    fn get_vector_mut(&mut self) -> &mut Vector3d {
        &mut self.v
    }
}
