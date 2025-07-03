use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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

    let pixels = Arc::new(Mutex::new(vec![
        renderer::RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        };
        WIDTH * HEIGHT
    ]));

    let parsed_data = Arc::new(parsed_data);
    let pixels_clone = Arc::clone(&pixels);
    let parsed_data_clone = Arc::clone(&parsed_data);

    thread::spawn(move || {
        loop {
            {
                let mut pixels = pixels_clone.lock().unwrap();
                draw_triangles(&mut pixels, &parsed_data_clone);
            }
            thread::sleep(Duration::from_secs(1));
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
