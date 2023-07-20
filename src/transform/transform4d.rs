use crate::matrix::Matrix4d;
use crate::transform::Point3D;
use crate::transform::Vector3D;
use crate::vector::Vector3d;
use crate::vector::vector_3d_operations::{cross, dot};

#[derive(Default, Debug, Clone)]
pub struct Transform4d {
    m: Matrix4d,
}

impl Transform4d {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        n00: f64, n01: f64, n02: f64, n03: f64,
        n10: f64, n11: f64, n12: f64, n13: f64,
        n20: f64, n21: f64, n22: f64, n23: f64,
    ) -> Self {
        Transform4d {
            m: {
                Matrix4d::new(
                    n00, n01, n02, n03,
                    n10, n11, n12, n13,
                    n20, n21, n22, n23,
                    0f64, 0f64, 0f64, 1f64,
                )
            },
        }
    }

    pub fn from_vectors(a: &Vector3D, b: &Vector3D, c: &Vector3D, p: &Point3D) -> Self {
        let a = &a.get_vector().to_vector_4d(0f64);
        let b = &b.get_vector().to_vector_4d(0f64);
        let c = &c.get_vector().to_vector_4d(0f64);
        let p = &p.get_vector().to_vector_4d(1f64);
        Transform4d {
            m: Matrix4d::from_vectors(a, b, c, p),
        }
    }

    pub fn row(&self, row_index: usize) -> &[f64; 4] {
        self.m.row(row_index)
    }

    pub fn row_mut(&mut self, row_index: usize) -> &mut [f64; 4] {
        self.m.row_mut(row_index)
    }

    pub fn element(&self, i: usize, j: usize) -> &f64 {
        self.m.element(i, j)
    }

    pub fn element_mutable(&mut self, i: usize, j: usize) -> &mut f64 {
        self.m.element_mutable(i, j)
    }

    pub fn get_translation(&self) -> &Point3D {
        unsafe { &*(&self[0] as *const Vector3d as *const Point3D) }
    }

    pub fn set_translation(&mut self, p: &Point3D) {
        self[3] = p.get_vector().clone();
    }

    pub fn inverse(&self) -> Transform4d {
        let a = &self[0];
        let b = &self[1];
        let c = &self[2];
        let d = &self[3];

        let mut s = cross(a, b);
        let mut t = cross(c, d);

        let inv_det = 1.0f64 / dot(&s, c);

        s *= inv_det;
        t *= inv_det;

        let v = c * inv_det;

        let r0 = cross(b, &v);
        let r1 = cross(&v, a);

        Transform4d::new(
            r0.x, r0.y, r0.z, -dot(b, &t),
            r1.x, r1.y, r1.z, dot(a, &t),
            s.x, s.y, s.z, -dot(d, &s),
        )
    }
}
