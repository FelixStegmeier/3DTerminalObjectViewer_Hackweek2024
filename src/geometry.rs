use std::ops::{self, Index, IndexMut};

#[derive(Clone, PartialEq)]
pub struct TransformMatrix {
    pub row_1: Vector3,
    pub row_2: Vector3,
    pub row_3: Vector3,
}
impl std::fmt::Display for TransformMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}\n{}\n", self.row_1, self.row_2, self.row_3)
    }
}
impl ops::Mul<Vector3> for TransformMatrix {
    type Output = Vector3;
    fn mul(self, vec: Vector3) -> Self::Output {
        Vector3 {
            x: self.row_1.x * vec.x + self.row_1.y * vec.y + self.row_1.z * vec.z,
            y: self.row_2.x * vec.x + self.row_2.y * vec.y + self.row_2.z * vec.z,
            z: self.row_3.x * vec.x + self.row_3.y * vec.y + self.row_3.z * vec.z,
        }
    }
}
struct _Plane {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}
impl std::fmt::Display for _Plane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Plane: {}x {}y {}z {}", self.a, self.b, self.c, self.d)
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl PartialEq<Vector3> for Vector3 {
    fn eq(&self, vec: &Vector3) -> bool {
        self.x == vec.x && self.y == vec.y && self.z == vec.z
    }
}
impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!(),
        }
    }
}

type Point = Vector3;
impl Index<usize> for Vector3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(),
        }
    }
}
impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, scalar: f64) -> Self::Output {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}
impl ops::Add for Vector3 {
    type Output = Vector3;
    fn add(self, vec: Vector3) -> Self::Output {
        Vector3 {
            x: self.x + vec.x,
            y: self.y + vec.y,
            z: self.z + vec.z,
        }
    }
}
impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}
const _UNIT_MATRIX: TransformMatrix = TransformMatrix {
    row_1: Vector3 {
        x: 1.,
        y: 0.,
        z: 0.,
    },
    row_2: Vector3 {
        x: 0.,
        y: 1.,
        z: 0.,
    },
    row_3: Vector3 {
        x: 0.,
        y: 0.,
        z: 1.,
    },
};

#[derive(Clone)]
pub struct Object {
    pub origin: Point,
    pub vertices: Vec<[f64; 3]>,
    pub _transform_vertices: Vec<[f64; 3]>,
    pub polygons: Vec<[usize; 3]>,
    pub _transform_matrix: TransformMatrix,
    pub _tranlation_vector: Vector3,
}
impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Object:\n        Origin: {}\n        Vertices: {:?}\n        Polygon list: {:?}\n\n",
            self.origin, self.vertices, self.polygons
        )
    }
}
impl ops::Mul for TransformMatrix {
    type Output = TransformMatrix;
    fn mul(self, matrix_2: Self) -> Self::Output {
        let mut sum: f64;
        let mut rows: [Vector3; 3] = [Vector3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }; 3];
        for row_index in 0..3 {
            let mut v: Vector3 = Vector3 {
                x: 0.,
                y: 0.,
                z: 0.,
            };
            for col_index in 0..3 {
                sum = 0.;
                for i in 0..3 {
                    // println!(
                    //     "sum: {} += {} * {}",
                    //     sum, self[row_index][i], matrix_2[i][col_index]
                    // );
                    sum += self[row_index][i] * matrix_2[i][col_index];
                }
                v[col_index] = sum;
            }
            //print!("\nrow index: {}; new row: {}\n", row_index, v);
            rows[row_index] = v;
        }
        TransformMatrix {
            row_1: rows[0],
            row_2: rows[1],
            row_3: rows[2],
        }
    }
}

impl Index<usize> for TransformMatrix {
    type Output = Vector3;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.row_1,
            1 => &self.row_2,
            2 => &self.row_3,
            _ => panic!(),
        }
    }
}
impl ops::Mul<[f64; 3]> for TransformMatrix {
    type Output = [f64; 3];
    fn mul(self, vec: [f64; 3]) -> Self::Output {
        [
            self.row_1.x * vec[0] + self.row_1.y * vec[0] + self.row_1.z * vec[0],
            self.row_2.x * vec[1] + self.row_2.y * vec[1] + self.row_2.z * vec[1],
            self.row_3.x * vec[2] + self.row_3.y * vec[2] + self.row_3.z * vec[2],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_matrix_matrix_mult() {
        let m1 = TransformMatrix {
            row_1: Vector3 {
                x: -1.,
                y: -1.,
                z: 2.,
            },
            row_2: Vector3 {
                x: 1.,
                y: -1.,
                z: 2.,
            },
            row_3: Vector3 {
                x: 3.,
                y: -3.,
                z: 2.,
            },
        };
        let m2 = TransformMatrix {
            row_1: Vector3 {
                x: 2.,
                y: 3.,
                z: 1.,
            },
            row_2: Vector3 {
                x: -1.,
                y: -2.,
                z: 1.,
            },
            row_3: Vector3 {
                x: 3.,
                y: -3.,
                z: 2.,
            },
        };
        let m_res = TransformMatrix {
            row_1: Vector3 {
                x: 5.,
                y: -7.,
                z: 2.,
            },
            row_2: Vector3 {
                x: 9.,
                y: -1.,
                z: 4.,
            },
            row_3: Vector3 {
                x: 15.,
                y: 9.,
                z: 4.,
            },
        };
        assert_eq!(m1 * m2 == m_res, true);
    }
    #[test]
    fn test_add_for_vector3_1() {
        assert_eq!(
            Vector3 {
                x: 0.,
                y: 0.,
                z: 0.
            } + Vector3 {
                x: 0.,
                y: 0.,
                z: 0.
            } == Vector3 {
                x: 0.,
                y: 0.,
                z: 0.
            },
            true
        );
    }
    #[test]
    fn test_add_for_vector3_2() {
        assert_eq!(
            Vector3 {
                x: 4.,
                y: -4.,
                z: 3.
            } + Vector3 {
                x: 7.,
                y: 2.,
                z: 3.
            } == Vector3 {
                x: 11.,
                y: -2.,
                z: 6.
            },
            true
        );
    }
    #[test]
    fn test_mul_for_f64_vector3() {
        assert_eq!(
            Vector3 {
                x: 7.,
                y: 2.,
                z: 3.
            } * 2.
                == Vector3 {
                    x: 14.,
                    y: 4.,
                    z: 6.
                },
            true
        );
    }
    #[test]
    fn test_mul_for_vector3_transformmatrix() {
        assert_eq!(
            TransformMatrix {
                row_1: Vector3 {
                    x: 6.,
                    y: 2.,
                    z: 4.,
                },
                row_2: Vector3 {
                    x: -1.,
                    y: 4.,
                    z: 3.,
                },
                row_3: Vector3 {
                    x: -2.,
                    y: 9.,
                    z: 3.,
                },
            } * Vector3 {
                x: 4.,
                y: -2.,
                z: 1.
            } == Vector3 {
                x: 24.,
                y: -9.,
                z: -23.
            },
            true
        );
    }
}
