pub mod vector;
use vector::vector_3d::Vector3d;
use vector::vector_3d_operations;
use vector::vector_4d::Vector4d;

pub mod matrix;
use matrix::matrix_3d::Matrix3d;





fn main() {
    let mut x: Vector3d = Vector3d::new(0f64, 0.0, 0f64);
    let mut y: Vector3d = Vector3d::new(0f64, 0f64, 0f64);
    let mut z: Vector3d = Vector3d::new(0f64,0f64,0f64);
    let mut m=Matrix3d::from_vectors(&x,&y,&z);
    
    let mut v=&mut m[0];
    v.x=13.0;
    
    //let what2=what[0];
    
    //let m2=m.inverse();
    
    
    //let v : Vector4d = Vector4d::new(1f64,2f64,3f64,5f64);
    
    //let z =   v.as_vector3d();
    //let y= unsafe { &*z };


    println!("{:?}",v);
    println!("{:?}",m);
    
    

    // 
    // let v1=Vector3d::new(1f64,0f64,0f64);
    // let v2=Vector3d::new(0f64,1f64,0f64);
    // let v3=vector_3d_operations::Cross(&v1,&v2);
    // 
    // let res=vector_3d_operations::Dot(&v1,&v2);
    // let res_1=vector_3d_operations::Dot(&v1,&v3);
    // let res_2=vector_3d_operations::Dot(&v2,&v3);
    // 
    // println!("dot of {:?} and {:?} is {res}",v1,v2);
    // 
    // 
    // println!("dot of {:?} and {:?} is {res_1}",v1,v3);
    // println!("dot of {:?} and {:?} is {res_2}",v2,v3);
    // 
    // let mat=Matrix3d::from_vectors(&x,&y,&z);
    // let mat_2=mat.clone();
    // 
    // println!("a lovely matrix {:?}",mat);
    // 
    // let v1=mat[0];
    // let v2=mat[1];
    // let v3=mat[2];
    // let v4 =v1+v2;
    // 
    // 
    // println!("{:?}",v1);
    // println!("{:?}",v2);
    // println!("{:?}",v3);
    
    //let mat3=mat*mat_2;
    
}