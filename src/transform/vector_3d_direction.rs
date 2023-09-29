
use crate::transform::transform_vector::{TransformVector, VectorWrapper};
use crate::vector::Vector3d;

pub type Vector3D = TransformVector<Vector3dDirection>;

#[derive(Default, Debug, Clone)]
#[repr(transparent)]
pub struct Vector3dDirection {
    pub v: Vector3d,
}

impl VectorWrapper for Vector3dDirection {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3dDirection {
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
