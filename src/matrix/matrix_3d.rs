use crate::vector::vector_3d::Vector3d;
use crate::vector::vector_3d_operations::{Cross, Dot};

#[derive(Default, Debug, Clone, Copy)]
pub struct Matrix3d {
    n: [[f64; 3]; 3],
}

impl Matrix3d {
    pub fn new(n00: f64, n01: f64, n02: f64,
               n10: f64, n11: f64, n12: f64,
               n20: f64, n21: f64, n22: f64,
    ) -> Self {
        Matrix3d {
            n: [
                [n00, n10, n20],
                [n01, n11, n21],
                [n02, n12, n22]
            ],
        }
    }

    pub fn from_vectors(a: &Vector3d, b: &Vector3d, c: &Vector3d) -> Self {
        Matrix3d {
            n: [
                [a.x, a.y, a.z],
                [b.x, b.y, b.z],
                [c.x, c.y, c.z]
            ],
        }
    }

    pub fn row(&self, row_index: usize) -> &[f64; 3] {
        &self.n[row_index]
    }

    pub fn row_mut(&mut self, row_index: usize) -> &mut [f64; 3] {
        &mut self.n[row_index]
    }

    pub fn element(&self, i: usize, j: usize) -> &f64 {
        &self.n[j][i]
    }

    pub fn element_mutable(&mut self, i: usize, j: usize) -> &mut f64 {
        &mut self.n[j][i]
    }

    pub fn determinant(&self) -> f64 {
        return
          self.element(0, 0) * (self.element(1, 1) * self.element(2, 2) - self.element(1, 2) * self.element(2, 1)) 
        + self.element(0, 1) * (self.element(1, 2) * self.element(2, 0) - self.element(1, 0) * self.element(2, 2)) 
        + self.element(0, 2) * (self.element(1, 0) * self.element(2, 1) - self.element(1, 1) * self.element(2, 0));
    }
    
    pub fn inverse(&self) -> Matrix3d {
        let a = &self[0];
        let b = &self[1];
        let c = &self[2];

        let r0 = Cross(b, c);
        let r1 = Cross(c, a);
        let r2 = Cross(a, b);
        
        let invDet=1f64/ Dot(&r2,&c);
        
        return Matrix3d::new(
            r0.x * invDet, r0.y * invDet, r0.z * invDet,
            r1.x * invDet, r1.y * invDet, r1.z * invDet,
            r2.x * invDet, r2.y * invDet, r2.z * invDet);
    }
}