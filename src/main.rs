use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;

struct Renderer {
    window: Window,
    pixels: Pixels,
}

impl Renderer {
    fn new(event_loop: &ActiveEventLoop) -> Result<Self, Error> {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
        let window_attributes = WindowAttributes::default()
            .with_title("Renderer")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size);

        let window = event_loop.create_window(window_attributes).unwrap();
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;

        Ok(Self { window, pixels })
    }

    fn render(&mut self) -> Result<(), Error> {
        let frame = self.pixels.frame_mut();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let offset = ((y * WIDTH + x) * 4) as usize;
                frame[offset] = x as u8; // R
                frame[offset + 1] = y as u8; // G
                frame[offset + 2] = 128; // B
                frame[offset + 3] = 255; // A
            }
        }

        self.pixels.render()?;
        self.window.request_redraw();

        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.pixels.resize_surface(width, height);
    }
}

struct App {
    renderer: Option<Renderer>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.renderer = Some(Renderer::new(event_loop).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::RedrawRequested => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.render().unwrap();
                }
            }

            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size.width, size.height);
                }
            }

            _ => (),
        }
    }
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App { renderer: None };
    _ = event_loop.run_app(&mut app);

    Ok(())
}
