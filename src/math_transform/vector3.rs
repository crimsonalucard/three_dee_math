pub mod vector3 {
    use std::ops::{Neg, Add, Sub, Mul, Index, IndexMut};
    use std::cmp::Eq;
    use std::fmt::{Display, Formatter, Result, Debug};

    #[derive(Clone, Copy)]
    pub struct Vector3 {
        pub(crate) x: f64,
        pub(crate) y: f64,
        pub(crate) z: f64,
    }


    impl Display for Vector3 {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "({},{},{})", self.x, self.y, self.z)
        }
    }

    impl Debug for Vector3 {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("Vector3")
                .field("x", &self.x)
                .field("y", &self.y)
                .field("z", &self.z)
                .finish()
        }
    }

    impl PartialEq for Vector3 {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y && self.z == other.z
        }
    }

    impl Eq for Vector3 {}

    impl Add<Vector3> for Vector3 {
        type Output = Vector3;
        fn add(self, rhs: Vector3) -> Self::Output {
            Vector3 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }

    impl Add<&Vector3> for Vector3 {
        type Output = Vector3;
        fn add(self, rhs: &Vector3) -> Self::Output {
            Vector3 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }

    impl Index<usize> for Vector3 {
        type Output = f64;
        fn index(&self, i: usize) -> &Self::Output {
            match i {
                0 => &self.x,
                1 => &self.y,
                2 => &self.z,
                3 => &1.0,
                _ => {
                    panic!("Index out of bounds")
                }
            }
        }
    }

    impl IndexMut<usize> for Vector3 {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            match index {
                0 => &mut self.x,
                1 => &mut self.y,
                2 => &mut self.z,
                _ => {
                    panic!("Index out of bounds")
                }
            }
        }
    }

    impl Neg for Vector3 {
        type Output = Vector3;
        fn neg(self) -> Self::Output {
            Vector3 {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            }
        }
    }

    impl Neg for &Vector3 {
        type Output = Vector3;
        fn neg(self) -> Self::Output {
            Vector3 {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            }
        }
    }

    impl Sub<Vector3> for Vector3 {
        type Output = Vector3;

        fn sub(self, rhs: Vector3) -> Self::Output {
            self + (-rhs)
        }
    }

    impl Sub<&Vector3> for Vector3 {
        type Output = Vector3;

        fn sub(self, rhs: &Vector3) -> Self::Output {
            self + (-rhs)
        }
    }

    impl Mul<Vector3> for Vector3 {
        type Output = f64;

        fn mul(self, rhs: Vector3) -> Self::Output {
            self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
        }
    }

    impl Mul<f64> for Vector3 {
        type Output = Vector3;

        fn mul(self, rhs: f64) -> Self::Output {
            Vector3 {
                x: rhs * self.x,
                y: rhs * self.y,
                z: rhs * self.z,
            }
        }
    }

    pub fn cross(a: Vector3, b: Vector3) -> Vector3 {
        Vector3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[test]
    fn test_vector_equality() {
        let a = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let b = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        assert_eq!(a, b);
        assert_eq!(a, a);
        assert_eq!(b, a);
    }

    #[test]
    fn test_addition() {
        let a = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let b = Vector3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };
        assert_eq!(a + a, b)
    }

    #[test]
    fn test_subtraction() {
        let a = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let b = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        assert_eq!(a - a, b)
    }

    #[test]
    fn test_dot_product() {
        let a = Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let b = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        assert_eq!(a * b, 0.0)
    }

    #[test]
    fn test_cross_product() {
        let a = Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };

        let b = Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        let c = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };

        assert_eq!(cross(a, b), c)
    }
}