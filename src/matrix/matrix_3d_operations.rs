use std::ops::{Index, IndexMut, Mul};

use crate::matrix::Matrix3d;
use crate::vector::Vector3d;

impl Index<usize> for Matrix3d {
    type Output = Vector3d;

    fn index(&self, index: usize) -> &Self::Output {
        if index > 2 {
            panic!("Out of 3d matrix range");
        }

        self.row(index).as_ref()
    }
}

impl IndexMut<usize> for Matrix3d {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > 2 {
            panic!("Out of 3d matrix range");
        }

        self.row_mut(index).as_mut()
    }
}

impl Mul<Matrix3d> for Matrix3d {
    type Output = Matrix3d;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix3d::new(
            self.element(0, 0) * rhs.element(0, 0) + self.element(0, 1) * rhs.element(1, 0) + self.element(0, 2) * rhs.element(2, 0),
            self.element(0, 0) * rhs.element(0, 1) + self.element(0, 1) * rhs.element(1, 1) + self.element(0, 2) * rhs.element(2, 1),
            self.element(0, 0) * rhs.element(0, 2) + self.element(0, 1) * rhs.element(1, 2) + self.element(0, 2) * rhs.element(2, 2),
            self.element(1, 0) * rhs.element(0, 0) + self.element(1, 1) * rhs.element(1, 0) + self.element(1, 2) * rhs.element(2, 0),
            self.element(1, 0) * rhs.element(0, 1) + self.element(1, 1) * rhs.element(1, 1) + self.element(1, 2) * rhs.element(2, 1),
            self.element(1, 0) * rhs.element(0, 2) + self.element(1, 1) * rhs.element(1, 2) + self.element(1, 2) * rhs.element(2, 2),
            self.element(2, 0) * rhs.element(0, 0) + self.element(2, 1) * rhs.element(1, 0) + self.element(2, 2) * rhs.element(2, 0),
            self.element(2, 0) * rhs.element(0, 1) + self.element(2, 1) * rhs.element(1, 1) + self.element(2, 2) * rhs.element(2, 1),
            self.element(2, 0) * rhs.element(0, 2) + self.element(2, 1) * rhs.element(1, 2) + self.element(2, 2) * rhs.element(2, 2),
        )
    }
}

impl Mul<Vector3d> for Matrix3d {
    type Output = Vector3d;

    fn mul(self, rhs: Vector3d) -> Self::Output {
        Vector3d::new(
            self.element(0, 0) * rhs.x + self.element(0, 1) * rhs.y + self.element(0, 2) * rhs.z,
            self.element(1, 0) * rhs.x + self.element(1, 1) * rhs.y + self.element(1, 2) * rhs.z,
            self.element(2, 0) * rhs.x + self.element(2, 1) * rhs.y + self.element(2, 2) * rhs.z,
        )
    }
}
