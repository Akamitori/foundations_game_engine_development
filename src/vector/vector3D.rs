#[derive(Default, Debug, Clone)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn empty() -> Self {
        Vector3D::new(0f64, 0f64, 0f64)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D {
            x,
            y,
            z,
        }
    }

    pub fn Magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    pub fn Normalize(&self) -> Vector3D{
        return  self.clone()/self.Magnitude()
    }
}

