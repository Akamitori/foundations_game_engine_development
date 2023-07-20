use std::ops::{Index, IndexMut, Mul};

use crate::transform::Point3D;
use crate::transform::Transform4d;
use crate::transform::Vector3D;
use crate::vector::Vector3d;

impl Index<usize> for Transform4d {
    type Output = Vector3d;

    fn index(&self, index: usize) -> &Self::Output {
        if index > 3 {
            panic!("Out of 4d matrix range");
        }

        let r = self.row(index).as_ptr() as *const Vector3d;

        unsafe { &*r }
    }
}

impl IndexMut<usize> for Transform4d {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > 3 {
            panic!("Out of 4d matrix range");
        }

        let r = self.row_mut(index).as_mut_ptr() as *mut Vector3d;

        unsafe { &mut *r }
    }
}

impl Mul for Transform4d {
    type Output = Transform4d;

    fn mul(self, rhs: Self) -> Self::Output {
        let n00 = self.element(0, 0) * rhs.element(0, 0)
            + self.element(0, 1) * rhs.element(1, 0)
            + self.element(0, 2) * rhs.element(2, 0);
        
        let n01 = self.element(0, 0) * rhs.element(0, 1)
            + self.element(0, 1) * rhs.element(1, 1)
            + self.element(0, 2) * rhs.element(2, 1);
        
        let n02 = self.element(0, 0) * rhs.element(0, 2)
            + self.element(0, 1) * rhs.element(1, 2)
            + self.element(0, 2) * rhs.element(2, 2);
        
        let n03 = self.element(0, 0) * rhs.element(0, 3)
            + self.element(0, 1) * rhs.element(1, 3)
            + self.element(0, 2) * rhs.element(2, 3)
            + self.element(0, 3);
        
        let n10 = self.element(1, 0) * rhs.element(0, 0)
            + self.element(1, 1) * rhs.element(1, 0)
            + self.element(1, 2) * rhs.element(2, 0);
        
        let n11 = self.element(1, 0) * rhs.element(0, 1)
            + self.element(1, 1) * rhs.element(1, 1)
            + self.element(1, 2) * rhs.element(2, 1);
        
        let n12 = self.element(1, 0) * rhs.element(0, 2)
            + self.element(1, 1) * rhs.element(1, 2)
            + self.element(1, 2) * rhs.element(2, 2);
        
        let n13 = self.element(1, 0) * rhs.element(0, 3)
            + self.element(1, 1) * rhs.element(1, 3)
            + self.element(1, 2) * rhs.element(2, 3)
            + self.element(1, 3);
        
        let n20 = self.element(2, 0) * rhs.element(0, 0)
            + self.element(2, 1) * rhs.element(1, 0)
            + self.element(2, 2) * rhs.element(2, 0);
        
        let n21 = self.element(2, 0) * rhs.element(0, 1)
            + self.element(2, 1) * rhs.element(1, 1)
            + self.element(2, 2) * rhs.element(2, 1);
        
        let n22 = self.element(2, 0) * rhs.element(0, 2)
            + self.element(2, 1) * rhs.element(1, 2)
            + self.element(2, 2) * rhs.element(2, 2);
        
        let n23 =
            self.element(2, 0) * rhs.element(0, 3)
                + self.element(2, 1) * rhs.element(1, 3)
                + self.element(2, 2) * rhs.element(2, 3)
                + self.element(2, 3);

        Transform4d::new(
            n00, n01, n02, n03,
            n10, n11, n12, n13,
            n20, n21, n22, n23,
        )
    }
}

impl Mul<Vector3D> for Transform4d {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        let v = rhs.get_vector();
        return Vector3D::new(
            self.element(0, 0) * v.x + self.element(0, 1) * v.y + self.element(0, 2) * v.z,
            self.element(1, 0) * v.x + self.element(1, 1) * v.y + self.element(1, 2) * v.z,
            self.element(2, 0) * v.x + self.element(2, 1) * v.y + self.element(2, 2) * v.z,
        );
    }
}

impl Mul<Point3D> for Transform4d {
    type Output = Point3D;

    fn mul(self, rhs: Point3D) -> Self::Output {
        let v = rhs.get_vector();
        return Point3D::new(
            self.element(0, 0) * v.x + self.element(0, 1) * v.y + self.element(0, 2) * v.z,
            self.element(1, 0) * v.x + self.element(1, 1) * v.y + self.element(1, 2) * v.z,
            self.element(2, 0) * v.x + self.element(2, 1) * v.y + self.element(2, 2) * v.z,
        );
    }
}
