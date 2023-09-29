use crate::matrix::Matrix3d;
use crate::transform::transform_vector::{TransformVector, VectorWrapper};

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct Quaternion {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
    pub(crate) w: f64,
}

impl Quaternion {
    pub fn new(a: f64, b: f64, c: f64, s: f64) -> Self {
        Quaternion {
            x: a,
            y: b,
            z: c,
            w: s,
        }
    }

    pub fn from_vector<T: VectorWrapper>(v: TransformVector<T>, s: f64) -> Self {
        let v = v.get_vector();
        Quaternion {
            x: v.x,
            y: v.y,
            z: v.z,
            w: s,
        }
    }

    pub fn get_vector_part<T: VectorWrapper>(&self) -> &TransformVector<T> {
        self.as_ref()
    }

    pub fn get_vector_part_mut<T: VectorWrapper>(&mut self) -> &mut TransformVector<T> {
        self.as_mut()
    }

    pub fn get_rotation_matrix(&self) -> Matrix3d {
        let x2 = self.x * self.x;
        let y2 = self.y * self.y;
        let z2 = self.z * self.z;
        let xy = self.x * self.y;
        let xz = self.x * self.z;
        let yz = self.y * self.z;
        let wx = self.w * self.x;
        let wy = self.w * self.y;
        let wz = self.w * self.z;

        Matrix3d::new(
            1f64 - 2f64 * (y2 + z2), 2f64 * (xy - wz), 2f64 * (xz + wy),
            2f64 * (xy + wz), 1f64 - 2f64 * (x2 + z2), 2f64 * (yz - wx),
            2f64 * (xz - wy), 2f64 * (yz + wx), 1f64 - 2f64 * (x2 + y2))
    }

    pub fn set_rotation_matrix(&mut self, m: &Matrix3d)
    {
        let m00 = m.element(0, 0);
        let m11 = m.element(1, 1);
        let m22 = m.element(2, 2);
        let sum = m00 + m11 + m22;

        if sum > 0f64
        {
            self.w = (sum + 1f64).sqrt() * 0.5;
            let f = 0.25 / self.w;

            self.x = (m.element(2, 1) - m.element(1, 2)) * f;
            self.y = (m.element(0, 2) - m.element(2, 0)) * f;
            self.z = (m.element(1, 0) - m.element(0, 1)) * f;
        } else if m00 > m11 && m00 > m22
        {
            self.x = (m00 - m11 - m22 + 1f64).sqrt() * 0.5;
            let f = 0.25 / self.x;

            self.y = (m.element(1, 0) + m.element(0, 1)) * f;
            self.z = (m.element(0, 2) + m.element(2, 0)) * f;
            self.w = (m.element(2, 1) - m.element(1, 2)) * f;
        } else if m11 > m22
        {
            self.y = (m11 - m00 - m22 + 1f64).sqrt() * 0.5;
            let f = 0.25 / self.y;

            self.x = (m.element(1, 0) + m.element(0, 1)) * f;
            self.z = (m.element(2, 1) + m.element(1, 2)) * f;
            self.w = (m.element(0, 2) - m.element(2, 0)) * f;
        } else {
            self.z = (m22 - m00 - m11 + 1f64).sqrt() * 0.5;
            let f = 0.25 / self.z;

            self.x = (m.element(0, 2) + m.element(2, 0)) * f;
            self.y = (m.element(2, 1) + m.element(1, 2)) * f;
            self.w = (m.element(1, 0) - m.element(0, 1)) * f;
        }
    }
}