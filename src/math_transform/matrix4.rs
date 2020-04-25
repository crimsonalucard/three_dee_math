pub mod matrix4 {
    use std::ops::{Index, Add, Sub, Mul, Neg, IndexMut};
    use std::cmp::{Eq, PartialEq};
    use std::fmt::{Debug, Result, Formatter};

    #[derive(Clone, Copy)]
    pub struct Matrix4 {
        pub(crate) matrix: [[f64; 4]; 4]
    }

    impl Index<usize> for Matrix4 {
        type Output = [f64; 4];

        fn index(&self, i: usize) -> &[f64; 4] {
            &self.matrix[i]
        }
    }

    impl IndexMut<usize> for Matrix4 {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.matrix[index]
        }
    }

    impl Debug for Matrix4 {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("Vector3")
                .field("matrix", &self.matrix)
                .finish()
        }
    }

    impl PartialEq for Matrix4 {
        fn eq(&self, other: &Self) -> bool {
            for i in 0..4 {
                for j in 0..4 {
                    if self[i][j] != other[i][j] {
                        return false;
                    }
                }
            }
            true
        }
    }

    impl Eq for Matrix4 {}

    impl Add<Matrix4> for Matrix4 {
        type Output = Matrix4;

        fn add(self, rhs: Matrix4) -> Matrix4 {
            Matrix4 {
                matrix: [
                    [self[0][0] + rhs[0][0], self[0][1] + rhs[0][1], self[0][2] + rhs[0][2], self[0][3] + rhs[0][3]],
                    [self[1][0] + rhs[1][0], self[1][1] + rhs[1][1], self[1][2] + rhs[1][2], self[1][3] + rhs[1][3]],
                    [self[2][0] + rhs[2][0], self[2][1] + rhs[2][1], self[2][2] + rhs[2][2], self[2][3] + rhs[2][3]],
                    [self[3][0] + rhs[3][0], self[3][1] + rhs[3][1], self[3][2] + rhs[3][2], self[3][3] + rhs[3][3]]
                ]
            }
        }
    }

    impl Mul<Matrix4> for Matrix4 {
        type Output = Matrix4;

        fn mul(self, rhs: Matrix4) -> Matrix4 {
            let mut result: Matrix4 = zero();
            for i in 0..4 {
                for j in 0..4 {
                    for k in 0..4 {
                        result[i][j] += self[i][k] * rhs[k][j];
                    }
                }
            }
            let result = result;
            result
        }
    }

    impl Mul<f64> for Matrix4 {
        type Output = Matrix4;

        fn mul(self, rhs: f64) -> Matrix4 {
            let mut result: Matrix4 = identity();
            for i in 0..4 {
                for j in 0..4 {
                    result[i][j] = self[i][j] * rhs;
                }
            }
            let result = result;
            result
        }
    }

    impl Neg for Matrix4 {
        type Output = Matrix4;

        fn neg(self) -> Self::Output {
            Matrix4 {
                matrix: [
                    [-self[0][0], -self[0][1], -self[0][2], -self[0][3]],
                    [-self[1][0], -self[1][1], -self[1][2], -self[1][3]],
                    [-self[2][0], -self[2][1], -self[2][2], -self[2][3]],
                    [-self[3][0], -self[3][1], -self[3][2], -self[3][3]]
                ]
            }
        }
    }

    impl Sub<Matrix4> for Matrix4 {
        type Output = Matrix4;

        fn sub(self, rhs: Matrix4) -> Self::Output {
            self + (-rhs)
        }
    }

    pub fn create_translation_matrix(x: f64, y: f64, z: f64) -> Matrix4 {
        Matrix4 {
            matrix: [
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }

    pub fn create_scale_matrix(x: f64, y: f64, z: f64) -> Matrix4 {
        Matrix4 {
            matrix: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }

    pub fn create_rotation_x(ax: f64) -> Matrix4 {
        Matrix4 {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, (-ax).cos(), -((-ax).sin()), 0.0],
                [0.0, (-ax).sin(), (-ax).cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }

    pub fn create_rotation_y(ay: f64) -> Matrix4 {
        Matrix4 {
            matrix: [
                [(-ay).cos(), 0.0, -((-ay).sin()), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-((-ay).sin()), 0.0, (-ay).cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }

    pub fn create_rotation_z(az: f64) -> Matrix4 {
        Matrix4 {
            matrix: [
                [(-az).cos(), -((-az).sin()), 0.0, 0.0],
                [(-az).sin(), (-az).cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }

    pub fn transpose(x: Matrix4) -> Matrix4 {
        Matrix4 {
            matrix: [
                [x[0][0], x[1][0], x[2][0], x[3][0]],
                [x[0][1], x[1][1], x[2][1], x[3][1]],
                [x[0][2], x[1][2], x[2][2], x[3][2]],
                [x[0][3], x[1][3], x[2][3], x[3][3]],
            ]
        }
    }

    pub fn zero() -> Matrix4 {
        Matrix4 {
            matrix: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0]
            ]
        }
    }

    pub fn identity() -> Matrix4 {
        Matrix4 {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        }
    }

    #[test]
    fn test_equality() {
        assert_eq!(identity(), identity())
    }

    #[test]
    fn test_addition() {
        assert_eq!(identity() + identity(), identity() * 2.0)
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(identity() - identity(), zero());
        assert_eq!(identity() - zero(), identity())
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(identity() * identity(), identity());
        assert_eq!(zero() * identity(), zero())
    }

    #[test]
    fn test_transpose(){
        assert_eq!(transpose(identity()), identity());
        assert_eq!(transpose(zero()), zero())
    }
}
