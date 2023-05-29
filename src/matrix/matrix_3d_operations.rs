use std::ops::{Index, IndexMut};

use crate::Matrix3d;
use crate::vector::vector_3d::Vector3d;

impl Index<usize> for Matrix3d {
    type Output = Vector3d;

    fn index(&self, index: usize) -> &Self::Output {
        if index > 3 {
            panic!("Out of 3d matrix range");
        }

        let r = self.row(index).as_ptr() as *const Vector3d;

        return  unsafe { &*r };
    }
}

impl IndexMut<usize> for Matrix3d {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > 3 {
            panic!("Out of 3d matrix range");
        }

        let r = self.row_mut(index).as_mut_ptr() as *mut Vector3d;

        return  unsafe { &mut *r };
    }
}