pub mod vector3;
pub mod matrix4;
pub mod quaternion;

use vector3::vector3::{Vector3, zero};
use matrix4::matrix4::{Matrix4, create_translation_matrix};
use quaternion::quaternion::{Quaternion, normalize};
use crate::math_transform::matrix4::matrix4::{identity, create_scale_matrix};
use std::intrinsics::sqrtf64;
use crate::math_transform::quaternion::quaternion::conjugate;

pub fn matrix_vector_transform(v: Vector3, m: Matrix4) -> Vector3 {
    let mut result = zero();
    for i in 0..4 {
        for j in 0..4 {
            if i != 3 {
                result[i] += (if j == 3 { 1.0 } else { v[j] }) * m[i][j]
            }
        }
    }
    let result = result;
    result
}

fn quaternion_from_axis_angle(axis: Vector3, angle: f64) -> Quaternion {
    Quaternion {
        w: (angle / 2.0).cos(),
        x: axis.x * (angle / 2.0).sin(),
        y: axis.y * (angle / 2.0).sin(),
        z: axis.z * (angle / 2.0).sin(),
    }
}

fn quaternion_to_axis_angle(q: Quaternion) -> (Vector3, f64) {
    let angle = 2 * (q.w).acos();
    let s = (1.0 - q.w * q.w).sqrt();
    if s < 0.001 {
        (Vector3 {
            x: q.x,
            y: q.y,
            z: q.z,
        }, angle)
    } else {
        (Vector3 {
            x: q.x / s,
            y: q.y / s,
            z: q.z / s,
        }, angle)
    }
}


fn quaternion_to_matrix(q: Quaternion) -> Matrix4 {
    Matrix4 {
        matrix: [
            [1.0 - 2.0 * q.y * q.y - 2.0 * q.z * q.z, 2.0 * q.x * q.y - 2.0 * q.z * q.w, 2.0 * q.x * q.z + 2.0 * q.y * q.w, 0.0],
            [2.0 * q.x * q.y + 2.0 * q.z * q.w, 1.0 - 2.0 * q.x * q.x - 2.0 * q.z * q.z, 2.0 * q.y * q.z - 2.0 * q.x * q.w, 0.0],
            [2.0 * q.x * q.z - 2.0 * q.y * q.w, 2.0 * q.y * q.z + 2.0 * q.x * q.w, 1.0 - 2.0 * q.x * q.x - 2.0 * q.y * q.y, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    }
}

fn matrix_to_quaternion(m: Matrix4) -> Quaternion {
    let mut qx = (m[0][0] + m[1][1] + m[2][2] + 1.0) / 4.0;
    let mut qy = (m[0][0] - m[1][1] - m[2][2] + 1.0) / 4.0;
    let mut qz = (-m[0][0] + m[1][1] - m[2][2] + 1.0) / 4.0;
    let mut qw = (-m[0][0] - m[1][1] + m[2][2] + 1.0) / 4.0;
    qx = if qx < 0.0 { 0.0 } else { qx };
    qy = if qy < 0.0 { 0.0 } else { qy };
    qz = if qz < 0.0 { 0.0 } else { qz };
    qw = if qw < 0.0 { 0.0 } else { qw };
    qx = qx.sqrt();
    qy = qy.sqrt();
    qz = qz.sqrt();
    qw = qz.sqrt();
    let sign = |x: f64| -> f64 { if x >= 0.0 { 1.0 } else { -1.0 } };
    if qx >= qy && qx >= qz && qx >= qw {
        qx *= 1.0;
        qy *= sign(m[2][1] - m[1][2]);
        qz *= sign(m[0][2] - m[2][0]);
        qw *= sign(m[1][0] - m[0][1]);
    } else if qy >= qx && qy >= qz && qy >= qw {
        qx *= sign(m[2][1] - m[1][2]);
        qy *= 1.0;
        qz *= sign(m[1][0] + m[0][1]);
        qw *= sign(m[0][2] + m[0][2]);
    } else if qw >= qx && qz >= qy && qz >= qw {
        qx *= sign(m[0][2] - m[2][0]);
        qy *= sign(m[1][0] + m[0][1]);
        qz *= 1.0;
        qw *= sign(m[2][1] + m[1][2]);
    } else if qw >= qx && qw >= qy && qw >= qz {
        qx *= sign(m[1][0] - m[0][1]);
        qy *= sign(m[2][0] + m[0][2]);
        qz *= sign(m[2][1] + m[1][2]);
        qw *= 1.0;
    } else {
        panic!("case error!")
    }
    let qx = qx;
    let qy = qy;
    let qz = qz;
    let qw = qw;
    normalize(Quaternion {
        w: qw,
        x: qx,
        y: qy,
        z: qz
    })
}

fn transform(v: Vector3, q: Quaternion) -> Vector3 {
    let qr = q * Quaternion{
        w: 0.0,
        x: v.x,
        y: v.y,
        z: v.z
    } * conjugate(q);
    Vector3{
        x: qr.x,
        y: qr.y,
        z: qr.z

    }
}


#[test]
fn test_matrix_transform() {
    let mut v = matrix_vector_transform(zero(), create_translation_matrix(1.0, 0.0, 0.0));
    v = matrix_vector_transform(v, create_translation_matrix(0.0, 1.0, 0.0));
    v = matrix_vector_transform(v, create_translation_matrix(0.0, 0.0, 1.0));
    let one = Vector3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    assert_eq!(v, one);

    assert_eq!(matrix_vector_transform(one, create_scale_matrix(2.0, 2.0, 2.0)), one * 2.0)
}