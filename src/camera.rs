use crate::geometry::{TransformMatrix, Vector3};
use crate::objects;
use core::panic;
use std::vec;

#[derive(Copy, Clone)]
pub struct CameraCorners {
    pub top_left: Vector3,
    pub top_right: Vector3,
    pub bottom_left: Vector3,
    pub bottom_right: Vector3,
}
impl std::fmt::Display for CameraCorners {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "\n        top left: {}, top right: {}\n        bottom left: {}, bottom right: {}",
            self.top_left, self.top_right, self.bottom_left, self.bottom_right
        )
    }
}
#[derive(Copy, Clone)]
pub struct Camera {
    pub position: Vector3,
    pub origin: Vector3,
    pub corners: CameraCorners,
}
impl std::fmt::Display for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Camera info:\n        Corners: {}\n\n        Origin: {}\n        Position: {}\n\n",
            self.corners, self.origin, self.position
        )
    }
}

///creates a new Main camera pointing in x direction
pub fn new_camera() -> Camera {
    let _position = Vector3 {
        x: 0.,
        y: 0.,
        z: -50.,
    };
    let _corners = initialize_camera_corners(_position, 30.);
    let main_camera = Camera {
        corners: CameraCorners {
            top_left: _corners[0],
            top_right: _corners[1],
            bottom_left: _corners[2],
            bottom_right: _corners[3],
        },

        position: _position,
        origin: Vector3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
    };
    return main_camera;
}

fn initialize_camera_corners(position: Vector3, distance: f64) -> [Vector3; 4] {
    //when initializing look in z direction
    let orientation = Vector3 {
        x: 0.,
        y: 0.,
        z: 1.,
    };
    let vec = orientation * distance;
    let center = position + vec;

    let offset_height = 20.;
    let offset_width = 20.;

    let top_left = Vector3 {
        x: center.x + offset_width,
        y: center.y + offset_height,
        z: center.z,
    };
    let top_right = Vector3 {
        x: center.x - offset_width,
        y: center.y + offset_height,
        z: center.z,
    };
    let bottom_left = Vector3 {
        x: center.x + offset_width,
        y: center.y - offset_height,
        z: center.z,
    };
    let bottom_right = Vector3 {
        x: center.x - offset_width,
        y: center.y - offset_height,
        z: center.z,
    };

    return [top_left, top_right, bottom_left, bottom_right];
}

///iterates over world coords in camera

pub fn raycasting(
    corner1: Vector3,
    corner2: Vector3,
    corner3: Vector3,
    corner4: Vector3,
    camera: Camera,
    obj: objects::Object,
) -> Vec<Vec<[u8; 3]>> {
    let mut map_2d: Vec<Vec<[u8; 3]>> = vec![];

    let mut ratio_a;
    let mut ratio_b;
    for i in 0..100 {
        let mut row: Vec<[u8; 3]> = vec![];
        //per new degree a new vec<>
        for j in 0..100 {
            ratio_a = j as f64 * 0.01;
            ratio_b = i as f64 * 0.01;
            //interpolating between the corners of the camera
            let vec_t = corner1 * (1. - ratio_a) + corner2 * ratio_a;
            let vec_b = corner3 * (1. - ratio_a) + corner4 * ratio_a;
            let point_p = vec_t * (1. - ratio_b) + vec_b * ratio_b;

            let vec_o = vector_ab(camera.position, point_p); //(point_p - camera.position;) //vec_o is the vector from camera position to point on viewport

            //for each polygon test vector vec_o get_intersection()
            //two possibilities:    if i care about the face(future proof) get the closest face
            //                      or just get the first intersection
            let mut color: [u8; 3] = [0, 0, 0];
            //checks if the line intersects an object
            for i in 0..obj.polygons.len() {
                let polygon = obj.polygons[i];
                let point_1 = Vector3 {
                    x: obj.transform_vertices[polygon[0]][0] + obj.origin.x,
                    y: obj.transform_vertices[polygon[0]][1] + obj.origin.y,
                    z: obj.transform_vertices[polygon[0]][2] + obj.origin.z,
                };
                let point_2 = Vector3 {
                    x: obj.transform_vertices[polygon[1]][0] + obj.origin.x,
                    y: obj.transform_vertices[polygon[1]][1] + obj.origin.y,
                    z: obj.transform_vertices[polygon[1]][2] + obj.origin.z,
                };
                let point_3 = Vector3 {
                    x: obj.transform_vertices[polygon[2]][0] + obj.origin.x,
                    y: obj.transform_vertices[polygon[2]][1] + obj.origin.y,
                    z: obj.transform_vertices[polygon[2]][2] + obj.origin.z,
                };
                let intersection =
                    get_intersection(point_1, point_2, point_3, vec_o, camera.position); //stimmt das so???
                let im_stupid_so_this_is_here = match intersection {
                    Some(_point) => [255, 255, 255],
                    None => [0, 0, 0], //hier continue
                };

                if im_stupid_so_this_is_here == [255, 255, 255] {
                    color = im_stupid_so_this_is_here;
                }
            }
            row.push(color);
            //push filled vector to outer vector
        }

        map_2d.push(row);
    }
    return map_2d;
}

///returns the vector if the ray intersects the plane inside the triangle
fn get_intersection(
    p1: Vector3, //p1-p3 are triangle corners
    p2: Vector3,
    p3: Vector3,
    vec_p: Vector3,                    //ray vector
    orig_p_and_camera_origin: Vector3, //point the ray "stands" on
) -> Option<Vector3> {
    let dir_vec_p = vector_to_unit_vector(vec_p);
    let plane_normal = cross_product(vector_ab(p1, p2), vector_ab(p3, p1));

    let normal = Vector3 {
        x: plane_normal.x,
        y: plane_normal.y,
        z: plane_normal.z,
    };

    let d: f64 = -1. * dot_product(normal, p3);

    let t = -(dot_product(normal, orig_p_and_camera_origin) + d) / dot_product(normal, dir_vec_p);
    if t < 0. {
        return None;
    }

    //resulting Position of the intersection:
    let p = orig_p_and_camera_origin + dir_vec_p * t;
    if point_inside_triangle(plane_normal, p1, p2, p3, p) {
        //order matters! 123 and 321 show very different results! probably because the normal is flipped
        return Some(p);
    }
    return None;
}
fn point_inside_triangle(
    //the issue probably arrises here...
    plane_normal: Vector3,
    triangle_p1: Vector3,
    triangle_p2: Vector3,
    triangle_p3: Vector3,
    point_on_plane: Vector3,
) -> bool {
    let _ = plane_normal;
    let v_12 = vector_ab(triangle_p2, triangle_p1);
    let v_23 = vector_ab(triangle_p3, triangle_p2);
    let v_31 = vector_ab(triangle_p1, triangle_p3);

    let v_1p = vector_ab(point_on_plane, triangle_p1);
    let v_2p = vector_ab(point_on_plane, triangle_p2);
    let v_3p = vector_ab(point_on_plane, triangle_p3);

    let v12_cross_v1p = cross_product(v_12, v_1p);
    let v23_cross_v2p = cross_product(v_23, v_2p);
    let v31_cross_v3p = cross_product(v_31, v_3p);

    let barycentric_a = dot_product(v12_cross_v1p, v23_cross_v2p);
    let barycentric_b = dot_product(v12_cross_v1p, v31_cross_v3p);
    let barycentric_c = dot_product(v31_cross_v3p, v23_cross_v2p);
    //if they have the same sign => point in triangle

    if barycentric_a > 0. && barycentric_b > 0. && barycentric_c > 0. {
        return true;
    } else {
        return false;
    }
}
fn vector_ab(a: Vector3, b: Vector3) -> Vector3 {
    Vector3 {
        x: b.x - a.x,
        y: b.y - a.y,
        z: b.z - a.z,
    }
}
fn cross_product(v_1: Vector3, v_2: Vector3) -> Vector3 {
    Vector3 {
        x: (v_1.y * v_2.z - v_2.y * v_1.z),
        y: -1. * (v_1.x * v_2.z - v_2.x * v_1.z),
        z: (v_1.x * v_2.y - v_2.x * v_1.y),
    }
}

fn dot_product(v_1: Vector3, v_2: Vector3) -> f64 {
    v_1.x * v_2.x + v_1.y * v_2.y + v_1.z * v_2.z
}

fn vector_to_unit_vector(vec: Vector3) -> Vector3 {
    let len = length_of_vector(vec);
    let vec = Vector3 {
        x: vec.x / len,
        y: vec.y / len,
        z: vec.z / len,
    };
    if vec.x.is_nan() || vec.y.is_nan() || vec.z.is_nan() {
        panic!();
    }
    vec
}
fn length_of_vector(vec: Vector3) -> f64 {
    let aux_vec = f64::sqrt(vec.x * vec.x + vec.y * vec.y);
    if aux_vec.is_nan() {
        panic!()
    }
    let res = f64::sqrt(aux_vec * aux_vec + vec.z * vec.z);
    if res.is_nan() {
        panic!()
    }
    res
}

fn turn_90_degrees(vec: Vector3) -> Vector3 {
    let transform_matrix = TransformMatrix {
        row_1: Vector3 {
            x: 0.,
            y: 0.,
            z: 1.,
        },
        row_2: Vector3 {
            x: 0.,
            y: 1.,
            z: 0.,
        },
        row_3: Vector3 {
            x: -1.,
            y: 0.,
            z: 0.,
        },
    };
    transform_matrix * vec
}
pub fn transform_camera(camera: &Camera) -> Camera {
    let origin = camera.origin;
    let position = turn_90_degrees(camera.position);
    let corners = CameraCorners {
        top_left: turn_90_degrees(camera.corners.top_left),
        top_right: turn_90_degrees(camera.corners.top_right),
        bottom_left: turn_90_degrees(camera.corners.bottom_left),
        bottom_right: turn_90_degrees(camera.corners.bottom_right),
    };
    Camera {
        corners: corners,
        position: position,
        origin: origin,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_point_inside_triangle_1() {
        let plane_normal = Vector3 {
            x: 0.,
            y: 0.,
            z: -800.,
        };
        let p1 = Vector3 {
            x: 10.,
            y: -10.,
            z: 100.,
        };
        let p2 = Vector3 {
            x: -10.,
            y: -10.,
            z: 100.,
        };
        let p3 = Vector3 {
            x: 0.,
            y: 10.,
            z: 100.,
        };
        let p = Vector3 {
            x: 0.,
            y: 0.,
            z: 100.,
        };
        assert_eq!(point_inside_triangle(plane_normal, p1, p2, p3, p), true);
    }
    #[test]
    fn test_point_inside_triangle_2() {
        let plane_normal = Vector3 {
            x: 0.,
            y: 0.,
            z: -800.,
        };
        let p1 = Vector3 {
            x: 10.,
            y: -10.,
            z: 100.,
        };
        let p2 = Vector3 {
            x: -10.,
            y: -10.,
            z: 100.,
        };
        let p3 = Vector3 {
            x: 0.,
            y: 10.,
            z: 100.,
        };
        let p = Vector3 {
            x: 9.,
            y: -9.,
            z: 100.,
        };
        assert_eq!(point_inside_triangle(plane_normal, p1, p2, p3, p), true);
    }
    #[test]
    fn test_point_inside_triangle_3() {
        let plane_normal = Vector3 {
            x: 0.,
            y: 0.,
            z: -800.,
        };
        let p1 = Vector3 {
            x: 10.,
            y: -10.,
            z: 100.,
        };
        let p2 = Vector3 {
            x: -10.,
            y: -10.,
            z: 100.,
        };
        let p3: Vector3 = Vector3 {
            x: 0.,
            y: 10.,
            z: 100.,
        };
        let p = Vector3 {
            x: 1000.,
            y: 0.,
            z: 99.,
        };
        assert_eq!(point_inside_triangle(plane_normal, p1, p2, p3, p), false);
    }
    #[test]
    fn test_get_intersection_1() {
        let p1 = Vector3 {
            x: 10.,
            y: -10.,
            z: 100.,
        };
        let p2 = Vector3 {
            x: -10.,
            y: -10.,
            z: 100.,
        };
        let p3 = Vector3 {
            x: 0.,
            y: 10.,
            z: 100.,
        };
        let vec_p: Vector3 = Vector3 {
            x: 0.,
            y: 0.,
            z: 1.,
        };
        let orig_p_and_camera_origin: Vector3 = Vector3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let res: Vector3 = match get_intersection(p1, p2, p3, vec_p, orig_p_and_camera_origin) {
            Some(some) => some,
            None => Vector3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
        };
        assert_ne!(
            res,
            Vector3 {
                x: 0.,
                y: 0.,
                z: 0.
            }
        )
    }
    #[test]
    fn test_get_intersection_2() {
        let vertices = vec![
            [1000., -1200., 0.],
            [-1000., -1200., 0.],
            [0., 400., 0.],
            [0., 0., 60.],
        ];
        let p1 = Vector3 {
            x: 1000.,
            y: -1200.,
            z: 3000.,
        };
        let p2 = Vector3 {
            x: -1000.,
            y: -1200.,
            z: 3000.,
        };
        let p3 = Vector3 {
            x: 0.,
            y: 400.,
            z: 3000.,
        };

        let res: Vector3 = match get_intersection(
            p1,
            p2,
            p3,
            Vector3 {
                x: 0.,
                y: -0.1,
                z: 100.,
            },
            Vector3 {
                x: 0.,
                y: 1.,
                z: 0.,
            },
        ) {
            Some(some) => some,
            None => Vector3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
        };
        assert_ne!(
            res,
            Vector3 {
                x: 0.,
                y: 0.,
                z: 0.
            }
        )
    }

    #[test]
    fn test_vector_to_unit_vector_1() {
        assert_eq!(
            vector_to_unit_vector(Vector3 {
                x: 16.,
                y: 0.,
                z: 0.
            }),
            Vector3 {
                x: 1.,
                y: 0.,
                z: 0.
            }
        );
        assert_eq!(
            vector_to_unit_vector(Vector3 {
                x: 0.,
                y: 340.3,
                z: 0.
            }),
            Vector3 {
                x: 0.,
                y: 1.,
                z: 0.
            }
        );
        assert_eq!(
            vector_to_unit_vector(Vector3 {
                x: 0.,
                y: 0.,
                z: -23.23
            }),
            Vector3 {
                x: 0.,
                y: 0.,
                z: -1.
            }
        );
    }

    #[test]
    fn test_length_of_vector_1() {
        assert_eq!(
            length_of_vector(Vector3 {
                x: 1.,
                y: 0.,
                z: 0.
            }),
            1.
        );
        assert_eq!(
            length_of_vector(Vector3 {
                x: 0.,
                y: 1.,
                z: 0.
            }),
            1.
        );
        assert_eq!(
            length_of_vector(Vector3 {
                x: 0.,
                y: 0.,
                z: 1.
            }),
            1.
        );
        assert_eq!(
            length_of_vector(Vector3 {
                x: -1.,
                y: 0.,
                z: 0.
            }),
            1.
        );
        assert_eq!(
            length_of_vector(Vector3 {
                x: 0.,
                y: 0.,
                z: 0.
            }),
            0.
        );
        assert_eq!(
            length_of_vector(Vector3 {
                x: 4.,
                y: 3.,
                z: 0.
            }),
            5.
        );
        assert_eq!(
            length_of_vector(Vector3 {
                x: 4.,
                y: 0.,
                z: -3.
            }),
            5.
        );
    }
    #[test]
    fn test_cross_product_1() {
        assert_eq!(
            cross_product(
                Vector3 {
                    x: 1.,
                    y: 2.,
                    z: 3.
                },
                Vector3 {
                    x: 4.,
                    y: 5.,
                    z: 6.
                }
            ),
            Vector3 {
                x: -3.,
                y: 6.,
                z: -3.
            }
        );
    }

    #[test]
    fn test_dot_product_1() {
        assert_eq!(
            dot_product(
                Vector3 {
                    x: 4.,
                    y: 0.,
                    z: 0.
                },
                Vector3 {
                    x: 4.,
                    y: 0.,
                    z: 0.
                }
            ),
            16.
        );
        assert_eq!(
            dot_product(
                Vector3 {
                    x: 4.,
                    y: 0.,
                    z: 0.
                },
                Vector3 {
                    x: -4.,
                    y: 0.,
                    z: 0.
                }
            ),
            -16.
        );
        assert_eq!(
            dot_product(
                Vector3 {
                    x: 4.,
                    y: 0.,
                    z: 0.
                },
                Vector3 {
                    x: 0.,
                    y: 0.,
                    z: 1.
                }
            ),
            0.
        );
    }
    #[test]
    fn test_vector_ab_1() {
        assert_eq!(
            vector_ab(
                Vector3 {
                    x: 4.,
                    y: 0.,
                    z: -3.
                },
                Vector3 {
                    x: 4.,
                    y: 0.,
                    z: -3.
                }
            ),
            Vector3 {
                x: 0.,
                y: 0.,
                z: 0.
            }
        );
        assert_eq!(
            vector_ab(
                Vector3 {
                    x: -4.,
                    y: 4.,
                    z: 25.
                },
                Vector3 {
                    x: 4.,
                    y: -2.,
                    z: -3.
                }
            ),
            Vector3 {
                x: 8.,
                y: -6.,
                z: -28.
            }
        );
        assert_eq!(
            vector_ab(
                Vector3 {
                    x: 42.,
                    y: 34.,
                    z: -33.
                },
                Vector3 {
                    x: 4.,
                    y: 0.,
                    z: -3.
                }
            ),
            Vector3 {
                x: -38.,
                y: -34.,
                z: 30.
            }
        );
        assert_eq!(
            vector_ab(
                Vector3 {
                    x: 4.,
                    y: 22.,
                    z: -32.
                },
                Vector3 {
                    x: 4.,
                    y: 12.,
                    z: -3.
                }
            ),
            Vector3 {
                x: 0.,
                y: -10.,
                z: 29.
            }
        );
    }
}
