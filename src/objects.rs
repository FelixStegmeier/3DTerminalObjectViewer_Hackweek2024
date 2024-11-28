use std::{ops, vec};

use crate::camera::Vector3;
// use crate::camera;
const _UNIT_MATRIX: TransformMatrix = TransformMatrix {
    col1: Point {
        x: 1.,
        y: 0.,
        z: 0.,
    },
    col2: Point {
        x: 0.,
        y: 1.,
        z: 0.,
    },
    col3: Point {
        x: 0.,
        y: 0.,
        z: 1.,
    },
};
#[derive(Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}
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

pub fn new_test_obj() -> Object {
    //points with origin as center:
    let vertices = vec![
        [20., -20., 50.],
        [-20., -20., 50.],
        [0., 20., 0.],
        [0., 0., -50.],
    ];
    let polygons = vec![
        [0, 1, 2],
        [0, 1, 3],
        [2, 3, 0], //2, 3, 0 != 0, 2, 3
        [1, 2, 3],
    ];
    let origin = Point {
        x: 0.,
        y: 0.,
        z: 100.,
    };

    // let vertices = vec![
    //     [20., 0., 100.], [-20., 0., 100.], [0., 20., 100.],[0., 10., 90.],
    // ];
    // let polygons = vec![[0, 1, 2], [0, 1, 3], [0, 2, 3], [1, 2, 3]];

    // let vertices = vec![[20., 0., 100.], [-20., 0., 100.], [0., 20., 100.]];
    // let polygons = vec![[0, 1, 2]];

    let obj = Object {
        origin: origin,
        vertices: vertices.clone(),
        transform_vertices: vertices,
        polygons: polygons,
        transform_matrix: _UNIT_MATRIX.clone(),
        tranlation_vector: Vector3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
    };
    return obj;
}

fn calc_center_of_triangles(vertices: Vec<[f64; 3]>, polygons: Vec<[usize; 3]>) -> Vec<[f64; 3]> {
    let mut center_points: Vec<[f64; 3]> = vec![];

    for polygon in polygons {
        let point_1 = vertices[polygon[0]];
        let point_2 = vertices[polygon[1]];
        let point_3 = vertices[polygon[2]];

        let vec_1_2 = vector_from_a_to_b(point_1.clone(), point_2.clone());
        let half_vec_1_2 = [vec_1_2[0] / 2., vec_1_2[0] / 2., vec_1_2[0] / 2.];
        let halfway_point = [
            point_1[0] + half_vec_1_2[0],
            point_1[1] + half_vec_1_2[1],
            point_1[2] + half_vec_1_2[2],
        ];

        let vec_halfway_point_ponint_3 = vector_from_a_to_b(halfway_point.clone(), point_3.clone());
        let center_point = [
            point_3[0] + vec_halfway_point_ponint_3[0],
            point_3[1] + vec_halfway_point_ponint_3[1],
            point_3[2] + vec_halfway_point_ponint_3[2],
        ];

        center_points.push(center_point);
        //halfway_point + halfway_point to ponint_3 / 2
    }
    return center_points;
}

fn vector_from_a_to_b(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    return [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
}

#[derive(Clone)]
pub struct TransformMatrix {
    //i know its stupid.... i should flip the whole thing
    pub col1: Point,
    pub col2: Point,
    pub col3: Point,
}
impl ops::Mul<Point> for TransformMatrix {
    type Output = Point;
    fn mul(self, vec: Point) -> Self::Output {
        Point {
            x: self.col1.x * vec.x + self.col2.x * vec.y + self.col3.x * vec.z,
            y: self.col1.y * vec.x + self.col2.y * vec.y + self.col3.y * vec.z,
            z: self.col1.z * vec.x + self.col2.z * vec.y + self.col3.z * vec.z,
        }
    }
}
impl ops::Mul for TransformMatrix {
    type Output = TransformMatrix;
    fn mul(self, vec: TransformMatrix) -> Self::Output {
        //yes... i wrote it out
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
    }
}

impl ops::Mul<[f64; 3]> for TransformMatrix {
    type Output = [f64; 3];
    fn mul(self, vec: [f64; 3]) -> Self::Output {
        [
            self.col1.x * vec[0] + self.col2.x * vec[1] + self.col3.x * vec[2],
            self.col1.y * vec[0] + self.col2.y * vec[1] + self.col3.y * vec[2],
            self.col1.z * vec[0] + self.col2.z * vec[1] + self.col3.z * vec[2],
        ]
    }
}
pub fn rotate(mut obj: Object, transform_matrix: TransformMatrix) -> Object {
    let mut new_vertices: Vec<[f64; 3]> = vec![];
    for vertex in obj.vertices.clone() {
        new_vertices.push(transform_matrix.clone() * vertex);
    }

    let new_transform_matrix = transform_matrix * obj.transform_matrix;

    for i in 0..obj.transform_vertices.len() {
        let vertex = obj.transform_vertices[i];
        obj.transform_vertices[i][0] = new_transform_matrix.col1.x * vertex[0]
            + new_transform_matrix.col2.x * vertex[1]
            + new_transform_matrix.col3.x * vertex[2];
        obj.transform_vertices[i][1] = new_transform_matrix.col1.y * vertex[0]
            + new_transform_matrix.col2.y * vertex[1]
            + new_transform_matrix.col3.y * vertex[2];
        obj.transform_vertices[i][2] = new_transform_matrix.col1.z * vertex[0]
            + new_transform_matrix.col2.z * vertex[1]
            + new_transform_matrix.col3.z * vertex[2];
    }
    obj.transform_matrix = new_transform_matrix;
    obj
    // Object {
    //     origin: obj.origin,
    //     vertices: obj.vertices,
    //     transform_vertices: obj.transform_vertices,/////////////////
    //     polygons: obj.polygons,
    //     transform_matrix: new_transform_matrix,
    //     tranlation_vector: obj.translation_vector,
    // }
}
