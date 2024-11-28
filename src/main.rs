mod camera;
mod objects;

///creates a single 'snapshot' buffer of chars
fn test_snapshot() {
    let mut obj: objects::Object = objects::new_test_obj();
    let main_camera = camera::new_camera();
    let one_degree_transform = objects::TransformMatrix {
        col1: objects::Point {
            x: 0.95,
            y: 0.,
            z: -0.0975,
        },
        col2: objects::Point {
            x: 0.,
            y: 1.,
            z: 0.,
        },
        col3: objects::Point {
            x: 0.0975,
            y: 0.,
            z: 0.95,
        },
    };
    loop {
        obj = objects::rotate(obj, one_degree_transform.clone());
        let vec_2d: Vec<Vec<[u8; 3]>> = camera::raycasting(
            main_camera.corners.top_left,
            main_camera.corners.top_right,
            main_camera.corners.bottom_left,
            main_camera.corners.bottom_right,
            main_camera,
            obj.clone(), //obj.clone(),
        );
        draw(&vec_2d);
        std::thread::sleep(time::Duration::from_millis(200));
    }
    fn draw(vec_2d: &Vec<Vec<[u8; 3]>>) {
        let mut string = "".to_owned();
        for col in vec_2d {
            for cell in col {
                //print!(" |{},{},{}| ", cell[0], cell[1], cell[2]);
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
}
fn draw(vec_2d: &Vec<Vec<[u8; 3]>>) -> String {
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
    string
}

use core::time;

// fn init(){
//     TimeDelta::new();
//     let obj: objects::Object = objects::newTestObj();
//     let mut main_camera = camera::newCamera();
//     print!("{}\n", main_camera);
//     print!("{}", obj);
//     let vec_2d: Vec<Vec<[u8; 3]>> = camera::raycasting(
//         main_camera.corners.top_left,
//         main_camera.corners.top_right,
//         main_camera.corners.bottom_left,
//         main_camera.corners.bottom_right,
//         main_camera,
//         obj.clone(),
//     );
//     print!(
//         "width of vec_2d: {} \nheight of vec_2d: {}\n",
//         vec_2d.len(),
//         vec_2d[0].len()
//     );

//     loop{
//         update()
//     }
// }
// fn update(){
//     draw(&vec_2d);

//     fn draw(vec_2d: &Vec<Vec<[u8; 3]>>) {
//         let mut string = "".to_owned();
//         for col in vec_2d {
//             for cell in col {
//                 //print!(" |{},{},{}| ", cell[0], cell[1], cell[2]);
//                 if *cell == [0, 0, 0] {
//                     string += "...";
//                 } else {
//                     string += "XXX";
//                 }
//             }
//             string += "\n";
//         }
//         print!("{}", string);
//     }
// }
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
        let one_degree_transform = objects::TransformMatrix {
            col1: objects::Point {
                x: 0.95,
                y: 0.,
                z: -0.0975,
            },
            col2: objects::Point {
                x: 0.,
                y: 1.,
                z: 0.,
            },
            col3: objects::Point {
                x: 0.0975,
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
        std::thread::sleep(time::Duration::from_millis(200));
    }
}
