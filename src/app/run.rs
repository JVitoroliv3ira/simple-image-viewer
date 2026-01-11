use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::{cli::Args, domain::ImageAsset};

use super::state::AppState;

pub fn run(image: ImageAsset, _args: &Args) -> Result<(), String> {
    let event_loop = match EventLoop::new() {
        Ok(e) => e,
        Err(e) => return Err(format!("Erro ao criar event loop: {e}")),
    };
    let ImageAsset {
        width,
        height,
        rgba,
    } = image;
    let mut state = AppState::new();

    let _ = event_loop.run(move |event, target| match event {
        Event::Resumed => {
            if state.window.is_none() {
                let w = WindowBuilder::new()
                    .with_title("Simple image viewer")
                    .with_inner_size(LogicalSize::new(width as f64, height as f64))
                    .build(target)
                    .unwrap();

                state.window_id = Some(w.id());
                state.window = Some(w);
            }

            let win = state.window.as_ref().unwrap();
            if state.pixels.is_none() {
                let size = win.inner_size();
                let surface = SurfaceTexture::new(size.width, size.height, win);
                state.pixels = Some(Pixels::new(width, height, surface).unwrap());
            }
            win.request_redraw();
        }
        Event::WindowEvent {
            window_id, event, ..
        } => {
            if !state.is_siv_window(window_id) {
                return;
            }

            match event {
                WindowEvent::CloseRequested => target.exit(),
                WindowEvent::Resized(new_size) => {
                    if let Some(p) = state.pixels.as_mut() {
                        let _ = p.resize_surface(new_size.width, new_size.height);
                    }
                    if let Some(w) = state.window.as_ref() {
                        w.request_redraw();
                    }
                }
                WindowEvent::RedrawRequested => {
                    if let Some(p) = state.pixels.as_mut() {
                        p.frame_mut().copy_from_slice(&rgba);
                        let _ = p.render();
                    }
                }
                _ => {}
            }
        }
        _ => {}
    });

    Ok(())
}
