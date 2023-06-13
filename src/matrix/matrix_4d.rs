use crate::vector::vector_3d_operations::{Cross, Dot};
use crate::vector::vector_4d::Vector4d;

#[derive(Default, Debug, Clone, Copy)]
pub struct Matrix4d {
    n: [[f64; 4]; 4],
}

impl Matrix4d {
    pub fn new(n00: f64, n01: f64, n02: f64, n03 : f64,
               n10: f64, n11: f64, n12: f64, n13 : f64,
               n20: f64, n21: f64, n22: f64, n23 : f64,
               n30: f64, n31: f64, n32: f64, n33 : f64,
    ) -> Self {
        Matrix4d {
            n: [
                [n00, n10, n20, n30],
                [n01, n11, n21, n31],
                [n02, n12, n22, n32],
                [n03, n13, n23, n33],
            ],
        }
    }

    pub fn from_vectors(a: &Vector4d, b: &Vector4d, c: &Vector4d, d: &Vector4d) -> Self {
        Matrix4d {
            n: [
                [a.x, a.y, a.z ,a.w],
                [b.x, b.y, b.z, b.w],
                [c.x, c.y, c.z, c.w],
                [d.x, d.y, d.z, d.w],
            ],
        }
    }

    pub fn row(&self, row_index: usize) -> &[f64; 4] {
        &self.n[row_index]
    }

    pub fn row_mut(&mut self, row_index: usize) -> &mut [f64; 4] {
        &mut self.n[row_index]
    }

    pub fn element(&self, i: usize, j: usize) -> &f64 {
        &self.n[j][i]
    }

    pub fn element_mutable(&mut self, i: usize, j: usize) -> &mut f64 {
        &mut self.n[j][i]
    }
    
    pub fn inverse(&self) -> Matrix4d {
        let a = self[0].as_vector3d();
        let b = self[1].as_vector3d();
        let c = self[2].as_vector3d();
        let d = self[3].as_vector3d();
        
    	let x = *self.element(3,0);
    	let y = *self.element(3,1);
    	let z = *self.element(3,2);
    	let w = *self.element(3,3);
    
    	let mut s = Cross(a, b);
    	let mut t = Cross(c, d);
    	let mut u = a * y - b * x;
    	let mut v = c * w - d * z;
    
    	let invDet = 1f64 / (Dot(&s, &v) + Dot(&t, &u));
    	s *= invDet;
    	t *= invDet;
    	u *= invDet;
    	v *= invDet;
    
    	let r0 = Cross(b, &v) + &t * y;
    	let r1 = Cross(&v, a) - &t * x;
    	let r2 = Cross(d, &u) + &s * w;
    	let r3 = Cross(&u, c) - &s * z;
    
    	return Matrix4d::new(r0.x, r0.y, r0.z, -Dot(b, &t),
    	                 r1.x, r1.y, r1.z,  Dot(a, &t),
    	                 r2.x, r2.y, r2.z, -Dot(d, &s),
    	                 r3.x, r3.y, r3.z,  Dot(c, &s));
    }
}