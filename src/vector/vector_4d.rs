use crate::vector::vector_3d::Vector3d;
use crate::vector::vector_3d_operations::Dot;

#[derive(Default, Debug, Clone, Copy)]
pub struct Vector4d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}



impl Vector4d {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Vector4d {
            x,
            y,
            z,
            w
        }
    }
    
    pub fn as_vector3d(&self) ->&Vector3d{
        return  unsafe {&*(self as *const Vector4d as *const Vector3d)};
    }
}