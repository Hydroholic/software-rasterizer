use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use std::error::Error;
use winit::event_loop::{ControlFlow, EventLoop};

use parser::parse_obj;
use vector::{draw_triangles, Model, Vector3};

pub mod parser;
pub mod renderer;
pub mod vector;

// TODO: Don't use globals
const WIDTH: usize = 720;
const HEIGHT: usize = 720;

#[derive(Clone)]
pub struct ColoredTriangle {
    pub triangle: vector::Triangle3,
    pub color: renderer::RGBA,
}

fn random_color() -> renderer::RGBA {
    renderer::RGBA {
        r: rand::random(),
        g: rand::random(),
        b: rand::random(),
        a: 255,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let monkey_file = include_str!("../resources/materials/monkey.obj");
    let cube_file = include_str!("../resources/materials/cube.obj");
    let monkey_triangles = parse_obj(monkey_file);
    let cube_triangles = parse_obj(cube_file);

    let pixels = Arc::new(Mutex::new(vec![
        renderer::RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 255
        };
        WIDTH * HEIGHT
    ]));

    let mut monkey_model: Model = monkey_triangles
        .into_iter()
        .map(|t| ColoredTriangle {
            triangle: t,
            color: random_color(),
        })
        .collect::<Vec<_>>().into();

    let mut cube_model: Model = cube_triangles
        .into_iter()
        .map(|t| ColoredTriangle {
            triangle: t,
            color: random_color(),
        })
        .collect::<Vec<_>>().into();

    let pixels_clone = Arc::clone(&pixels);

    thread::spawn(move || {
        let mut transform = vector::Transform {
            position: Vector3 {
                x: 0.1,
                y: 0.1,
                z: -5.0,
            },
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };
        let fov = 60.0;
        loop {
            {
                let start = std::time::Instant::now();
                let mut pixels = pixels_clone.lock().unwrap();
                pixels.fill(renderer::RGBA {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 255,
                });


                let transform_start = std::time::Instant::now();
                monkey_model.apply_transform(&transform);
                let transform_time = transform_start.elapsed();

                let draw_start = std::time::Instant::now();
                draw_triangles(&mut pixels, &monkey_model.triangles, fov);
                let draw_time = draw_start.elapsed();

                let elapsed = start.elapsed();
                print!(
                    "Frame rendered in: {:.2?} ({:.2?} drawing, {:.2?} transforming)",
                    elapsed, draw_time, transform_time
                );
                println!();
            }
            transform.direction.x += 0.02;
            transform.direction.y += 0.01;
            thread::sleep(Duration::from_millis(10));
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
