use crate::transform::Point3D;
use crate::transform::Vector3D;
use std::ops::{Add, Sub};

macro_rules! create_operation {
    (
        "Sub",
        $self_type : ty,
        $rhs_type : ty,
        $result_type : ty
    ) => {
        impl Sub<$rhs_type> for $self_type {
            type Output = $result_type;

            fn sub(self, rhs: $rhs_type) -> Self::Output {
                let self_vec = self.get_vector();
                let rhs_vec = rhs.get_vector();

                <$result_type>::new(
                    self_vec.x - rhs_vec.x,
                    self_vec.y - rhs_vec.y,
                    self_vec.z - rhs_vec.z,
                )
            }
        }
    };

    (
        "Add",
        $self_type : ty,
        $rhs_type : ty,
        $result_type : ty
    ) => {
        impl Add<$rhs_type> for $self_type {
            type Output = $result_type;

            fn add(self, rhs: $rhs_type) -> Self::Output {
                let self_vec = self.get_vector();
                let rhs_vec = rhs.get_vector();

                <$result_type>::new(
                    self_vec.x + rhs_vec.x,
                    self_vec.y + rhs_vec.y,
                    self_vec.z + rhs_vec.z,
                )
            }
        }
    };
}

create_operation!("Sub", Point3D, Point3D, Vector3D);
create_operation!("Sub", Point3D, &Point3D, Vector3D);
create_operation!("Sub", &Point3D, Point3D, Vector3D);
create_operation!("Sub", &Point3D, &Point3D, Vector3D);

create_operation!("Sub", Point3D, Vector3D, Point3D);
create_operation!("Sub", Point3D, &Vector3D, Point3D);
create_operation!("Sub", &Point3D, Vector3D, Point3D);
create_operation!("Sub", &Point3D, &Vector3D, Point3D);

create_operation!("Sub", Vector3D, Point3D, Vector3D);
create_operation!("Sub", Vector3D, &Point3D, Vector3D);
create_operation!("Sub", &Vector3D, Point3D, Vector3D);
create_operation!("Sub", &Vector3D, &Point3D, Vector3D);

create_operation!("Add", Point3D, Vector3D, Point3D);
create_operation!("Add", Point3D, &Vector3D, Point3D);
create_operation!("Add", &Point3D, Vector3D, Point3D);
create_operation!("Add", &Point3D, &Vector3D, Point3D);

create_operation!("Add", Vector3D, Point3D, Point3D);
create_operation!("Add", Vector3D, &Point3D, Point3D);
create_operation!("Add", &Vector3D, Point3D, Point3D);
create_operation!("Add", &Vector3D, &Point3D, Point3D);
