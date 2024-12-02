use std::ops::{self, Index};
use std::error::Error;

use chrono::OutOfRange;

enum IndexError{
    IndexOutOfBounds,
}
#[derive(Clone)]
struct TransformMatrix {
    row_1: Vector3,
    row_2: Vector3,
    row_3: Vector3,
}
impl ops::Mul<Vector3> for TransformMatrix {
    type Output = Vector3;
    fn mul(self, vec: Vector3) -> Self::Output {
        Vector3 {
            x: self.row_1.x * vec.x + self.row_2.x * vec.y + self.row_3.x * vec.z,
            y: self.row_1.y * vec.x + self.row_2.y * vec.y + self.row_3.y * vec.z,
            z: self.row_1.z * vec.x + self.row_2.z * vec.y + self.row_3.z * vec.z,
        }
    }
}
struct Plane {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}
impl std::fmt::Display for Plane {
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

type Point = Vector3;
impl Index<usize> for Vector3{
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output{
        match index{
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(), //////////////////////////////////////////////////fick dich doch
        }
    }
}

impl PartialEq<Vector3> for Vector3 {
    fn eq(&self, vec: &Vector3) -> bool {
        self.x == vec.x && self.y == vec.y && self.z == vec.z
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
    pub transform_vertices: Vec<[f64; 3]>,
    pub polygons: Vec<[usize; 3]>,
    pub transform_matrix: TransformMatrix,
    pub tranlation_vector: Vector3,
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

// impl ops::Mul<Point> for TransformMatrix {
//     type Output = Point;
//     fn mul(self, vec: Point) -> Self::Output {
//         Point {
//             x: self.col1.x * vec.x + self.col2.x * vec.y + self.col3.x * vec.z,
//             y: self.col1.y * vec.x + self.col2.y * vec.y + self.col3.y * vec.z,
//             z: self.col1.z * vec.x + self.col2.z * vec.y + self.col3.z * vec.z,
//         }
//     }
// }
impl ops::Mul for TransformMatrix {
    type Output = TransformMatrix;
    fn mul(self, matrix_2: Self) -> Self::Output {
        let mut sum: f64;
        let mut rows: [Vector3; 3] = [Vector3{x: 0., y: 0., z: 0.}; 3];
        for row_index in 0..3{
            let mut v: Vector3 = Vector3{x: 0., y: 0., z: 0.};
            for col_index in 0..3{
                sum = 0.;
                for i in 0..3 {
                    sum += self[row_index][i] * matrix_2[i][col_index];
                }
                v.x = sum;
            }
            rows[row_index] = v;
        }
        TransformMatrix{row_1: rows[0], row_2: rows[1], row_3: rows[2]}
    }
    /* fn mul(self, vec: TransformMatrix) -> Self::Output {

        TransformMatrix {
            col1: Point {
                x: (self.col1.x * vec.col1.x + self.col2.x * vec.col1.y + self.col3.x * vec.col1.z),
                y: (self.col1.y * vec.col1.x + self.col2.y * vec.col1.y + self.col3.y * vec.col1.z),
                z: (self.col1.z * vec.col1.x + self.col2.z * vec.col1.y + self.col3.z * vec.col1.z),
            },
            col2: Point {
                x: self.col1.x * vec.col2.x + self.col2.x * vec.col2.y + self.col3.x * vec.col2.z,
                y: self.col1.y * vec.col2.x + self.col2.y * vec.col2.y + self.col3.y * vec.col2.z,
                z: self.col1.z * vec.col2.x + self.col2.z * vec.col2.y + self.col3.z * vec.col2.z,
            },
            col3: Point {
                x: (self.col1.x * vec.col3.x + self.col2.x * vec.col3.y + self.col3.x * vec.col3.z),
                y: (self.col1.y * vec.col3.x + self.col2.y * vec.col3.y + self.col3.y * vec.col3.z),
                z: (self.col1.z * vec.col3.x + self.col2.z * vec.col3.y + self.col3.z * vec.col3.z),
            },
        }
    } */
    
}
///helper for matrix * matrix, calcs one cell
fn matrix_matrix_cell(matrix_1: TransformMatrix, matrix_2: TransformMatrix, row: usize, col: usize) -> f64{
    let mut sum = 0.;
    for i in 0..3 {
        sum += matrix_1[row][i] * matrix_2[i][col];
    }
    sum
}
impl Index<usize> for TransformMatrix{
    type Output = Vector3;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.row_1,
            1 => &self.row_2,
            2 => &self.row_3,
            _ => panic!()
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
// impl ops::Mul<[f64; 3]> for TransformMatrix {
//     type Output = [f64; 3];
//     fn mul(self, vec: [f64; 3]) -> Self::Output {
//         [
//             self.col1.x * vec[0] + self.col2.x * vec[1] + self.col3.x * vec[2],
//             self.col1.y * vec[0] + self.col2.y * vec[1] + self.col3.y * vec[2],
//             self.col1.z * vec[0] + self.col2.z * vec[1] + self.col3.z * vec[2],
//         ]
//     }
// }