use std::ops::Deref;
use volume_1::transform::transform4d::Transform4d;
use volume_1::transform::{Point3D, Vector3D};
use volume_1::vector::{Vector3d, Vector4d};

fn main() {
    let mut x = Vector3D::new(1f64, 2.0, 3f64);
    let mut y = Vector3D::new(4f64, 5f64, 7f64);
    let mut z = Vector3D::new(3f64, 2f64, 9f64);
    let mut w = Point3D::new(1f64, 2f64, 3f64);
    let mut w_2 = Point3D::new(2f64, 4f64, 6.5f64);
    //let mut m=Matrix4d::from_vectors(&x,&y,&z);

    //let z= &x+w_2;
    //let x=x*2f64;
    let mut w_3=-w_2;

    println!("{:?}",w_3);
}

pub fn xa(t: &Vector3d) {}
