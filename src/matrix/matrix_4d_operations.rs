use std::ops::{Index, IndexMut, Mul};

use crate::matrix::matrix_4d::Matrix4d;
use crate::vector::vector_4d::Vector4d;

impl Index<usize> for Matrix4d {
    type Output = Vector4d;

    fn index(&self, index: usize) -> &Self::Output {
        if index > 3 {
            panic!("Out of 4d matrix range");
        }

        let r = self.row(index).as_ptr() as *const Vector4d;

        return unsafe { &*r };
    }
}

impl IndexMut<usize> for Matrix4d {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > 3 {
            panic!("Out of 4d matrix range");
        }

        let r = self.row_mut(index).as_mut_ptr() as *mut Vector4d;

        return unsafe { &mut *r };
    }
}