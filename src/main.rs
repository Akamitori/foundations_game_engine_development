pub mod vector;
use vector::vector3D::Vector3D;

fn main() {
    
    let mut x :Vector3D= Vector3D::new(2f64,2f64,1f64); 
    let mut y : Vector3D = Vector3D::empty();
    
    y=x.clone();
    test(&mut x);
    
      //let
    
    let z=x.Normalize();
    
    //println!("Hello, world! {:?} with magnitude {}",-&x, z.Magnitude());
    //println!("Hello, world! {:?} with magnitude {}",-x, y.Magnitude());
    
    
    let mut c=z;
    c+=x;
    
    println!("{:?}",c);
    
}

fn test(v:  &mut Vector3D) {
    *v/=5.0;
}
