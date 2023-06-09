use crate::vector::vector_3d::Vector3d;

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
    
}