use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use std::error::Error;
use winit::event_loop::{ControlFlow, EventLoop};

use parser::parse_obj;
use vector::{Model, Vector3, draw_triangles};

pub mod parser;
pub mod renderer;
pub mod vector;

// TODO: Don't use globals
const WIDTH: usize = 720;
const HEIGHT: usize = 720;

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

    let monkey_transform = vector::Transform {
        position: Vector3 { x: 0.1, y: 0.1, z: -5.0 },
        direction: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    };

    let monkey_colored_triangles = monkey_triangles
        .into_iter()
        .map(|t| ColoredTriangle {
            triangle: t,
            color: random_color(),
        })
        .collect::<Vec<_>>();

    let mut monkey_model = Model {
        triangles: monkey_colored_triangles,
        transform: monkey_transform,
    };

    let cube_transform = vector::Transform {
        position: Vector3 {
            x: 0.0,
            y: 0.0,
            z: -7.0,
        },
        direction: Vector3 {
            x: 10.0,
            y: 0.0,
            z: 0.0,
        },
    };

    let cube_color_triangles = cube_triangles
        .into_iter()
        .map(|t| ColoredTriangle {
            triangle: t,
            color: random_color(),
        })
        .collect::<Vec<_>>();

    let mut cube_model = Model {
        triangles: cube_color_triangles,
        transform: cube_transform,
    };

    let camera_transform = vector::Transform::default();
    let camera = vector::Camera {
        fov: 60.0,
        transform: camera_transform,
    };

    let pixels_clone = Arc::clone(&pixels);

    thread::spawn(move || {
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

                let transformed_monkey_triangles = monkey_model.apply_transform();
                let transformed_cube_triangles = cube_model.apply_transform();
                let all_triangles =
                    [transformed_monkey_triangles, transformed_cube_triangles].concat();

                let draw_start = std::time::Instant::now();
                draw_triangles(&mut pixels, &all_triangles, camera.fov);
                let draw_time = draw_start.elapsed();

                let elapsed = start.elapsed();
                print!(
                    "Frame rendered in: {:.2?} ({:.2?} drawing)",
                    elapsed, draw_time
                );
                println!();
            }
            monkey_model.transform.direction.x += 0.02;
            monkey_model.transform.direction.y += 0.01;
            thread::sleep(Duration::from_millis(10));
        }
    });

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
