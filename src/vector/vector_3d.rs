use crate::vector::vector_3d_operations::dot;
use crate::vector::vector_4d::Vector4d;

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3d { x, y, z }
    }
    
    pub fn to_vector_4d(&self, w: f64) -> Vector4d {
        Vector4d::new(self.x, self.y, self.z, w)
    }

    #[inline(always)]
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[inline(always)]
    pub fn normalize(&self) -> Vector3d {
        self / self.magnitude()
    }

    #[inline(always)]
    pub fn project(&self, to: &Vector3d) -> Vector3d {
        to * dot(self, to) / dot(to, to)
    }

    #[inline(always)]
    pub fn reject(&self, from: &Vector3d) -> Vector3d {
        self - from * dot(self, from) / dot(from, from)
    }
}
