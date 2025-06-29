use std::{error::Error, sync::{Arc, Mutex}, thread, time::Duration};
use winit::event_loop::{ControlFlow, EventLoop};

use vector::Vector2;

pub mod renderer;
pub mod vector;

const WIDTH: usize = 128;
const HEIGHT: usize = 64;


fn triangle_image() -> Vec<renderer::RGBA> {
    let mut buffer = vec![renderer::RGBA { r: 0, g: 0, b: 0, a: 255 }; WIDTH * HEIGHT];

    let rf = { || rand::random_range(0f32..=1f32) };
    let color = renderer::RGBA {
        r: rand::random(),
        g: rand::random(),
        b: rand::random(),
        a: 255,
    };

    let a = Vector2 { x: rf(), y: rf() };
    let b = Vector2 { x: rf(), y: rf() };
    let c = Vector2 { x: rf(), y: rf() };

    let min_x = a.x.min(b.x).min(c.x);
    let min_y = a.y.min(b.y).min(c.y);
    let max_x = a.x.max(b.x).max(c.x);
    let max_y = a.y.max(b.y).max(c.y);

    let min_height = (min_y * (HEIGHT as f32)) as usize;
    let min_width = (min_x * (WIDTH as f32)) as usize;
    let max_height = (max_y * (HEIGHT as f32)).ceil() as usize;
    let max_width = (max_x * (WIDTH as f32)).ceil() as usize;


    for y in min_height..max_height {
        for x in min_width..max_width {
            let p = Vector2 {
                x: x as f32 / WIDTH as f32,
                y: y as f32 / HEIGHT as f32,
            };
            if vector::point_in_triangle(&a, &b, &c, &p) {
                buffer[y * WIDTH + x] = color.clone();
            }
        }
    }
    buffer
}

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let w_settings = renderer::WindowSettings {
        title: "Pixels Example".to_string(),
        width: WIDTH as u32,
        height: HEIGHT as u32,
    };

    // Use Arc<Mutex<...>> for shared mutable access
    let pixels_buffer = Arc::new(Mutex::new(triangle_image()));

    // Spawn a thread to update the image every second
    {
        let pixels_buffer = Arc::clone(&pixels_buffer);
        thread::spawn(move || {
            loop {
                let new_image = triangle_image(); // Or any image generation logic
                let mut buffer = pixels_buffer.lock().unwrap();
                *buffer = new_image;
                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    let mut app = renderer::App::new(w_settings, pixels_buffer.clone());
    _ = event_loop.run_app(&mut app);

    Ok(())
}
