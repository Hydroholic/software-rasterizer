use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use std::error::Error;
use winit::event_loop::{ControlFlow, EventLoop};

use parser::parse_obj;
use vector::{draw_triangles, get_quaternion, transform3, Triangle3, Vector3};

pub mod parser;
pub mod renderer;
pub mod vector;

const WIDTH: usize = 720;
const HEIGHT: usize = 720;

pub struct ColoredTriangle {
    pub triangle: vector::Triangle3,
    pub color: renderer::RGBA,
}

fn transform_colored_triangle(q0: &Vector3, q1: &Vector3, q2: &Vector3, t: &ColoredTriangle) -> ColoredTriangle {
    ColoredTriangle {
        triangle: Triangle3 {
            a: transform3(q0, q1, q2, &t.triangle.a),
            b: transform3(q0, q1, q2, &t.triangle.b),
            c: transform3(q0, q1, q2, &t.triangle.c),
        },
        color: t.color.clone(),
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let obj_data = include_str!("../resources/materials/cube.obj");
    let parsed_data = parse_obj(obj_data);

    let pixels = Arc::new(Mutex::new(vec![
        renderer::RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        };
        WIDTH * HEIGHT
    ]));

    let colored_triangles = parsed_data
        .into_iter()
        .map(|t| ColoredTriangle {
            triangle: t,
            color: renderer::RGBA {
                r: rand::random(),
                g: rand::random(),
                b: rand::random(),
                a: 255,
            },
        })
        .collect::<Vec<_>>();

    let colored_triangles = Arc::new(colored_triangles);
    let pixels_clone = Arc::clone(&pixels);
    let colored_triangles_clone = Arc::clone(&colored_triangles);

    thread::spawn(move || {
        let mut yaw = 0.0;
        loop {
            {
                let mut pixels = pixels_clone.lock().unwrap();
                let q = get_quaternion(yaw);
                let transformed_triangles = colored_triangles_clone
                    .iter()
                    .map(
                        |t| transform_colored_triangle(&q.0, &q.1, &q.2, t)
                    ).collect::<Vec<_>>();
                draw_triangles(&mut pixels, &transformed_triangles);
            }
            yaw += 0.1;
            thread::sleep(Duration::from_millis(100));
        }
    });

    struct Image {
        width: usize,
        height: usize,
        pixels: Arc<Mutex<Vec<renderer::RGBA>>>,
    }

    impl renderer::PixelProvider for Image {
        fn get_pixels(&self) -> Vec<renderer::RGBA> {
            self.pixels.lock().unwrap().clone()
        }

        fn width(&self) -> u32 {
            self.width as u32
        }

        fn height(&self) -> u32 {
            self.height as u32
        }
    }

    let image = Image {
        pixels,
        width: WIDTH,
        height: HEIGHT,
    };
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = renderer::App::new(&image);
    _ = event_loop.run_app(&mut app);

    Ok(())
}
