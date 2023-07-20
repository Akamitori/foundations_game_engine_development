use crate::vector::Vector3d;

#[derive(Default, Debug, Clone)]
pub struct Vector4d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector4d {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Vector4d { x, y, z, w }
    }

    pub fn as_vector3d(&self) -> &Vector3d {
        return unsafe { &*(self as *const Vector4d as *const Vector3d) };
    }

    pub fn as_vector3d_mut(&self) -> &mut Vector3d {
        return unsafe { &mut *(self as *const Vector4d as *mut Vector3d) };
    }
}
