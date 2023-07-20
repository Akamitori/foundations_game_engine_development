use crate::matrix::Matrix3d;
use crate::vector::Vector3d;

pub fn make_rotation_x(t: f64) -> Matrix3d {
    let c = t.cos();
    let s = t.sin();

    Matrix3d::new(
        1f64, 0f64, 0f64,
        0f64, c, -s,
        0f64, s, c,
    )
}

pub fn make_rotation_y(t: f64) -> Matrix3d {
    let c = t.cos();
    let s = t.sin();

    Matrix3d::new(
        c, 0f64, s,
        0f64, 1f64, 0f64,
        -s, 0f64, c,
    )
}

pub fn make_rotation_z(t: f64) -> Matrix3d {
    let c = t.cos();
    let s = t.sin();

    Matrix3d::new(
        c, -s, 0f64,
        s, c, 0f64,
        0f64, 0f64, 1f64,
    )
}

pub fn make_rotation(t: f64, a: &Vector3d) -> Matrix3d {
    let c = t.cos();
    let s = t.sin();
    let d = 1f64 - c;

    let y = a.y * d;
    let z = a.z * d;
    let x = a.x * d;
    let axay = x * a.y;
    let axaz = x * a.z;
    let ayaz = y * a.z;

    Matrix3d::new(
        c + x * a.x, axay - s * a.z, axaz + s * a.y,
        axay + s * a.z, c + y * a.y, ayaz - s * a.x,
        axaz - s * a.y, ayaz + s * a.x, c + z * a.z,
    )
}

pub fn make_reflection(a: &Vector3d) -> Matrix3d {
    let x = a.x * -2f64;
    let y = a.y * -2f64;
    let z = a.z * -2f64;
    let axay = x * a.y;
    let axaz = x * a.z;
    let ayaz = y * a.z;

    Matrix3d::new(
        x * a.x + 1f64, axay, axaz,
        axay, y * a.y + 1f64, ayaz,
        axaz, ayaz, z * a.z + 1f64)
}

pub fn make_involution(a: &Vector3d) {
    let x = a.x * 2f64;
    let y = a.y * 2f64;
    let z = a.z * 2f64;
    let axay = x * a.y;
    let axaz = x * a.z;
    let ayaz = y * a.z;

    Matrix3d::new(
        x * a.x - 1f64, axay, axaz,
        axay, y * a.y - 1f64, ayaz,
        axaz, ayaz, z * a.z - 1f64);
}

pub fn make_scale_along_xyz_axes(sx: f64, sy: f64, sz: f64) -> Matrix3d {
    Matrix3d::new(
        sx, 0f64, 0f64,
        0f64, sy, 0f64,
        0f64, 0f64, sz,
    )
}

pub fn make_scale_in_the_direction_of_a(s: f64, a: &Vector3d) -> Matrix3d {
    let s = s - 1f64;
    let x = a.x * s;
    let y = a.y * s;
    let z = a.z * s;
    let axay = x * a.y;
    let axaz = x * a.z;
    let ayaz = y * a.z;

    Matrix3d::new(
        x * a.x + 1f64, axay, axaz,
        axay, y * a.y + 1f64, ayaz,
        axaz, ayaz, z * a.z + 1f64)
}

pub fn make_skew(t: f64, a: &Vector3d, b: &Vector3d) -> Matrix3d {
    let t = t.tan();
    let x = a.x * t;
    let y = a.y * t;
    let z = a.z * t;

    Matrix3d::new(
        x * b.x + 1f64, x * b.y, x * b.z,
        y * b.x, y * b.y + 1f64, y * b.z,
        z * b.x, z * b.y, z * b.z + 1f64
    )
}
