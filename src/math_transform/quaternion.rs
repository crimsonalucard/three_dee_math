pub mod quaternion {
    use std::ops::{Add, Mul};
    use crate::math_transform::matrix_vector_transform;
    use crate::math_transform::vector3::vector3::{Vector3, cross};

    #[derive(Copy, Clone)]
    pub struct Quaternion {
        pub(crate) w: f64,
        pub(crate) x: f64,
        pub(crate) y: f64,
        pub(crate) z: f64,
    }

    impl Add<Quaternion> for Quaternion {
        type Output = Quaternion;
        fn add(self, rhs: Quaternion) -> Self::Output {
            Quaternion {
                w: self.w + rhs.w,
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }

    impl Mul<Quaternion> for Quaternion {
        type Output = Quaternion;
        fn mul(self, rhs: Quaternion) -> Self::Output {
            let va = Vector3 {
                x: self.x,
                y: self.y,
                z: self.z,
            };
            let vb = Vector3 {
                x: rhs.x,
                y: rhs.y,
                z: rhs.z,
            };

            let w = self.w * rhs.w - (va * vb);

            let vr = cross(va, vb) + vb * self.w + va * rhs.w;

            Quaternion {
                w,
                x: vr.x,
                y: vr.y,
                z: vr.z,
            }
        }
    }

    impl Mul<&Quaternion> for &Quaternion {
        type Output = &Quaternion;
        fn mul(self, rhs: &Quaternion) -> Self::Output {}
    }

    pub fn identity() -> Quaternion {
        Quaternion {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn conjugate(q: Quaternion) -> Quaternion {
        Quaternion {
            w: q.w,
            x: -q.x,
            y: -q.y,
            z: -q.z,
        }
    }

    pub fn magnitude_squared(q: Quaternion) -> f64 {
        q.w * q.w + q.x * q.x + q.y * q.y + q.z * q.z
    }

    pub fn normalize(q: Quaternion) -> Quaternion {
        let magnitude = magnitude_squared(q);
        Quaternion {
            w: q.w / magnitude,
            x: q.x / magnitude,
            y: q.y / magnitude,
            z: q.z / magnitude,
        }
    }
}