use core::time;
use geometry::{TransformMatrix, Vector3};

mod camera;
mod geometry;
mod objects;

///creates a single 'snapshot' buffer of chars
fn test_snapshot() {
    let mut obj: objects::Object = objects::new_test_obj();
    let main_camera = camera::new_camera();
    let one_degree_transform = TransformMatrix {
        row_1: Vector3 {
            x: 0.95,
            y: 0.,
            z: 0.0975,
        },
        row_2: Vector3 {
            x: 0.,
            y: 1.,
            z: 0.,
        },
        row_3: Vector3 {
            x: -0.0975,
            y: 0.,
            z: 0.95,
        },
    };

    loop {
        let vec_2d: Vec<Vec<[u8; 3]>> = camera::raycasting(
            main_camera.corners.top_left,
            main_camera.corners.top_right,
            main_camera.corners.bottom_left,
            main_camera.corners.bottom_right,
            main_camera,
            obj.clone(),
        );
        draw(&vec_2d);
        std::thread::sleep(time::Duration::from_millis(1000));
        obj = objects::rotate(obj, one_degree_transform.clone());
    }
}

fn draw(vec_2d: &Vec<Vec<[u8; 3]>>) {
    let mut string = "".to_owned();
    for col in vec_2d {
        for cell in col {
            if *cell == [0, 0, 0] {
                string += "...";
            } else {
                string += "XXX";
            }
        }
        string += "\n";
    }
    print!("{}", string);
}

fn main() {
    test_snapshot();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_draw_1() {
        let mut obj: objects::Object = objects::new_test_obj();
        let main_camera = camera::new_camera();
        let one_degree_transform = geometry::TransformMatrix {
            row_1: Vector3 {
                x: 0.95,
                y: 0.,
                z: 0.0975,
            },
            row_2: Vector3 {
                x: 0.,
                y: 1.,
                z: 0.,
            },
            row_3: Vector3 {
                x: -0.0975,
                y: 0.,
                z: 0.95,
            },
        };
        obj = objects::rotate(obj.clone(), one_degree_transform.clone());
        let vec_2d: Vec<Vec<[u8; 3]>> = camera::raycasting(
            main_camera.corners.top_left,
            main_camera.corners.top_right,
            main_camera.corners.bottom_left,
            main_camera.corners.bottom_right,
            main_camera,
            obj.clone(),
        );
        draw(&vec_2d);
    }
}
