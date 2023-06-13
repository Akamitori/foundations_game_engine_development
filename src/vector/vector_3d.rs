use crate::vector::vector_3d_operations::Dot;

#[derive(Default, Debug, Clone)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3d {
            x,
            y,
            z,
        }
    }

    #[inline(always)]
    pub fn magnitude(&self) -> f64 {
    (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[inline(always)]
    pub fn normalize(&self) -> Vector3d {
        return self/ self.magnitude();
    }

    #[inline(always)]
    pub fn Project(&self, to: &Vector3d) -> Vector3d {
        return to * Dot(self, to) / Dot(to, to);
    }

    #[inline(always)]
    pub fn Reject(&self, from: &Vector3d) -> Vector3d {
        return self - from * Dot(self, from) / Dot(from, from);
    }
}