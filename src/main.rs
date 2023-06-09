use std::borrow::Borrow;

use matrix::matrix_3d::Matrix3d;
use vector::vector_3d::Vector3d;
use crate::vector::vector_3d_operations;

pub mod vector;
pub mod matrix;


fn main() {
    let mut x: Vector3d = Vector3d::new(1f64, 2.0, 3.0);
    let mut y: Vector3d = Vector3d::new(4f64, 5f64, 6f64);
    let mut z: Vector3d = Vector3d::new(7f64, 8f64, 9f64);

   
    let v1=Vector3d::new(1f64,0f64,0f64);
    let v2=Vector3d::new(0f64,1f64,0f64);
    let v3=vector_3d_operations::Cross(&v1,&v2);
    
    let res=vector_3d_operations::Dot(&v1,&v2);
    let res_1=vector_3d_operations::Dot(&v1,&v3);
    let res_2=vector_3d_operations::Dot(&v2,&v3);
    
    println!("dot of {:?} and {:?} is {res}",v1,v2);


    println!("dot of {:?} and {:?} is {res_1}",v1,v3);
    println!("dot of {:?} and {:?} is {res_2}",v2,v3);
    
    let mat=Matrix3d::from_vectors(&x,&y,&z);
    let mat_2=mat.clone();
    
    println!("a lovely matrix {:?}",mat);
    
    let v1=mat[0];
    let v2=mat[1];
    let v3=mat[2];
    let v4 =v1+v2;
    
    
    println!("{:?}",v1);
    println!("{:?}",v2);
    println!("{:?}",v3);
    
    let mat3=mat*mat_2;
    
}