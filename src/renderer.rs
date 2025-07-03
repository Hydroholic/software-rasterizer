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

pub struct Image<'a> {
    width: u32,
    height: u32,
    pixels: &'a Vec<RGBA>,
}

impl<'a> Image<'a> {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> &'a [RGBA] {
        self.pixels
    }

    pub fn new(pixels: &'a Vec<RGBA>, width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels,
        }
    }
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
    fn new(event_loop: &ActiveEventLoop, window_settings: &WindowSettings) -> Self {
        let logical_size =
            LogicalSize::new(window_settings.width as f64, window_settings.height as f64);

        let window_attributes = WindowAttributes::default()
            .with_title(window_settings.title.clone())
            .with_inner_size(logical_size)
            .with_min_inner_size(logical_size);

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window.clone());
        let pixels = Pixels::new(
            window_settings.width,
            window_settings.height,
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

pub struct App<'a> {
    renderer: Option<Renderer>,
    window_settings: WindowSettings,
    image: Image<'a>,
}

impl<'a> App<'a> {
    pub fn new(window_settings: WindowSettings, image: Image<'a>) -> Self {
        Self {
            renderer: None,
            window_settings,
            image,
        }
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.renderer = Some(Renderer::new(event_loop, &self.window_settings));
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
                    let pixels = self.image.pixels;
                    renderer.render(
                        pixels,
                        self.window_settings.width,
                        self.window_settings.height,
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
