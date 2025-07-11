use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

pub struct WindowSettings {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

pub trait PixelProvider {
    fn get_pixels(&self) -> Vec<RGBA>;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

#[derive(Clone)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

struct Renderer {
    window: Arc<Window>,
    pixels: Pixels<'static>,
}

impl Renderer {
    fn new(event_loop: &ActiveEventLoop, image: &impl PixelProvider) -> Self {
        let logical_size =
            LogicalSize::new(image.width() as f64, image.height() as f64);

        let window_attributes = WindowAttributes::default()
            .with_title("Pixels Example".to_string())
            .with_inner_size(logical_size)
            .with_min_inner_size(logical_size);

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window.clone());
        let pixels = Pixels::new(
            image.width(),
            image.height(),
            surface_texture,
        )
        .expect("Failed to create Pixels instance");

        Self { window, pixels }
    }

    fn render(&mut self, pixels: &[RGBA], width: u32, height: u32) {
        let frame = self.pixels.frame_mut();

        for y in 0..height {
            for x in 0..width {
                let offset = (y * width + x) as usize;
                let offset_frame = ((y * width + x) * 4) as usize;
                frame[offset_frame] = pixels[offset].r;
                frame[offset_frame + 1] = pixels[offset].g;
                frame[offset_frame + 2] = pixels[offset].b;
                frame[offset_frame + 3] = pixels[offset].a;
            }
        }

        self.pixels.render().expect("Failed to render pixels");
        self.window.request_redraw();
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.pixels.resize_surface(width, height).unwrap();
    }
}

pub struct App<'a, P: PixelProvider> {
    renderer: Option<Renderer>,
    image: &'a P,
}

impl<'a, P: PixelProvider> App<'a, P> {
    pub fn new(image: &'a P) -> Self {
        Self {
            renderer: None,
            image,
        }
    }
}

impl<'a, P: PixelProvider> ApplicationHandler for App<'a, P> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.renderer = Some(Renderer::new(event_loop, self.image));
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
                    let pixels = self.image.get_pixels();
                    renderer.render(
                        &pixels,
                        self.image.width(),
                        self.image.height(),
                    );
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
