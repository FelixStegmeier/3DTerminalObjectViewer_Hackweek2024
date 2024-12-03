use crate::geometry::{TransformMatrix, Vector3};
use std::vec;
pub(crate) type Point = Vector3;

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
    pub vertices: Vec<Vector3>,
    pub transform_vertices: Vec<Vector3>,
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
        Vector3{x: 20., y: -20., z: 50.},
        Vector3{x: -20.,y:  -20.,z:  50.},
        Vector3{x: 0.,y: 20.,z: 0.},
        Vector3{x: 0.,y: 0.,z: -50.},
    ];
    let polygons = vec![[0, 1, 2], [0, 1, 3], [2, 3, 0], [1, 2, 3]];
    let origin = Point {
        x: 0.,
        y: 0.,
        z: 100.,
    };

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

pub fn rotate(mut obj: Object, transform_matrix: TransformMatrix) -> Object {
    let mut new_vertices: Vec<Vector3> = vec![];
    for vertex in obj.vertices.clone() {
        new_vertices.push(transform_matrix.clone() * vertex);
    }

    let new_transform_matrix = transform_matrix * obj.transform_matrix;

    for i in 0..obj.transform_vertices.len() {
        let vertex = obj.transform_vertices[i];
        obj.transform_vertices[i][0] = new_transform_matrix.row_1.x * vertex[0]
            + new_transform_matrix.row_1.y * vertex[1]
            + new_transform_matrix.row_1.z * vertex[2];
        obj.transform_vertices[i][1] = new_transform_matrix.row_2.x * vertex[0]
            + new_transform_matrix.row_2.y * vertex[1]
            + new_transform_matrix.row_2.z * vertex[2];
        obj.transform_vertices[i][2] = new_transform_matrix.row_3.x * vertex[0]
            + new_transform_matrix.row_3.y * vertex[1]
            + new_transform_matrix.row_3.z * vertex[2];
    }
    obj.transform_matrix = new_transform_matrix;
    obj
}
