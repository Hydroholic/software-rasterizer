use std::{fmt, sync::Arc};

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

struct RenderError(String);

impl fmt::Debug for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RenderError: {}", self.0)
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
    fn new(
        event_loop: &ActiveEventLoop,
        window_settings: &WindowSettings,
    ) -> Result<Self, RenderError> {
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
        .map_err(|err| RenderError(format!("Failed to create Pixels: {}", err)))?;

        Ok(Self { window, pixels })
    }

    fn render(&mut self, pixels: &[RGBA], width: u32, height: u32) -> Result<(), RenderError> {
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

        self.pixels
            .render()
            .map_err(|err| RenderError(format!("Failed to render pixels: {}", err)))?;
        self.window.request_redraw();

        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.pixels.resize_surface(width, height).unwrap();
    }
}

pub trait PixelBuffer: Send + Sync {
    fn get(&self) -> Vec<RGBA>;
}

pub struct App<B: PixelBuffer> {
    renderer: Option<Renderer>,
    window_settings: WindowSettings,
    pixels_buffer: B,
}

impl<B: PixelBuffer> App<B> {
    pub fn new(window_settings: WindowSettings, pixels_buffer: B) -> Self {
        Self {
            renderer: None,
            window_settings,
            pixels_buffer,
        }
    }
}

impl<B: PixelBuffer> ApplicationHandler for App<B> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.renderer = Some(Renderer::new(event_loop, &self.window_settings).unwrap());
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
                    renderer
                        .render(
                            &self.pixels_buffer.get(),
                            self.window_settings.width,
                            self.window_settings.height,
                        )
                        .unwrap();
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
