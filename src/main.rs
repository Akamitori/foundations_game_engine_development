use volume_1::quaternion::quaternion::Quaternion;
use volume_1::transform::{Point3D, Vector3D};
use volume_1::vector::{Vector3d};

fn main() {

    let _x = Vector3d::new(1f64, 2.0, 3f64);
    let _y = Vector3d::new(1f64, 2.0, 3f64);
    let _x=_x+_y;
    let _x = Vector3D::new(1f64, 2.0, 3f64);
    let _y = Vector3D::new(4f64, 5f64, 7f64);
    let _z = Vector3D::new(3f64, 2f64, 9f64);
    let _w = Point3D::new(1f64, 2f64, 3f64);
    let _w_2 = Point3D::new(2f64, 4f64, 6.5f64);
    //let mut m=Matrix4d::from_vectors(&x,&y,&z);

    //let z= &x+w_2;
    //let x=x*2f64;
    let _w_3=-_w_2;
    
    let mut _z=Quaternion::from_vector(_z,1.0);
    let mut _z2 : & mut Point3D=_z.get_vector_part_mut();
     
    _z2.0.v.x=13f64;

    println!("{:?}",_z);
}
