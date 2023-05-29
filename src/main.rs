

use vector::vector_3d::Vector3d;
use matrix::matrix_3d::Matrix3d;

pub mod vector;
pub mod matrix;

fn main() {
    let mut x: Vector3d = Vector3d::default();
    let mut y: Vector3d = Vector3d::new(2f64, 2f64, 1f64);
    let mut z: Vector3d = y*2f64;
    
    let adw=x[0];

    y = x.clone();
    test(&mut x);

    let mut m : Matrix3d = Matrix3d::from_vectors(
        &x,
        &y,
        &z
    );
    
    //m[0]= Vector3d::new(0f64,0f64,0f64);
    
    //let z=m[0];
    
    let mut a=m.element(0, 1);
    //m[0]= Vector3d::new(0f64,0f64,0f64);
    //let mut b=m.element_mutable(0, 1);
    
    //*b=70f64;
    
    println!("{:?}",a);
    println!("{:?}",m);
    // 
    // let mut sampleArray = [[0f64; 3]; 3];
    // 
    // for i in 0..3 {
    //     for j in 0..3 {
    //         sampleArray[i][j] = (i + j) as f64;
    //     }
    // }
    // 
    // println!("{:?}", sampleArray.clone());
    // 
    // 
    // // let x=sampleArray[5];
    // assert!(sampleArray[0].len() == 3);
    // let a = sampleArray[0].as_ptr() as * const Vector3D;
    // let b = sampleArray[0].as_mut_ptr() as *mut Vector3D;
    // 
    // let a = unsafe { &*a };
    // let b = unsafe { &mut *b };
    // 
    //  b.x=13.0;
    // 
    // 
    // 
    // println!("{:?} and {:?}",a,b);
    // // let z=sampleArray[0].as_mut_ptr() as *mut  Vector3D;
    // // let w: &mut Vector3D = unsafe { &mut *z };
    // // w.x=1.0;
    // // 
    // // println!("{:?}",w);
    // // println!("{:?}",sampleArray.clone());
    // // 
    // // 
    // // let a =Petros(2);
    // // let z = Mitsos(1);
    // // 
    // // let p=
    // 
    // 
    // //let size_1=mem::size_of::<[f64;3]>();
    // //let size_2=mem::size_of::<Vector3D>();
    // //let
    // 
    // //let z=x.Normalize();
    // 
    // //println!("Hello, world! {:?} with magnitude {}",-&x, z.Magnitude());
    // //println!("Hello, world! {:?} with magnitude {}",-x, y.Magnitude());
    // 
    // 
    // //let mut c=z;
    // //c+=x;
    // 
    // println!("{} {}",size_1,size_2);
}

fn test(v: &mut Vector3d) {
    *v /= 5.0;
}
