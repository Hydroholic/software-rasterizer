use std::error::Error;
use winit::event_loop::{ControlFlow, EventLoop};

use parser::parse_obj;
use vector::draw_triangles;

pub mod parser;
pub mod renderer;
pub mod vector;

const WIDTH: usize = 720;
const HEIGHT: usize = 720;

fn main() -> Result<(), Box<dyn Error>> {
    let obj_data = include_str!("../resources/materials/cube.obj");
    let parsed_data = parse_obj(obj_data);
    let mut pixels = vec![
        renderer::RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        };
        WIDTH * HEIGHT
    ];
    draw_triangles(&mut pixels, parsed_data);
    let image = renderer::Image::new(&pixels, WIDTH as u32, HEIGHT as u32);
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = renderer::App::new(image);
    _ = event_loop.run_app(&mut app);

    Ok(())
}
