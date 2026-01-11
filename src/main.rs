use image::GenericImageView;
use pixels::{Pixels, SurfaceTexture};
use std::{env, path::PathBuf};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder, WindowId},
};

fn main() {
    let mut args = env::args().skip(1);

    let Some(p) = args.next() else {
        eprintln!("Uso: siv <caminho-da-imagem>");
        std::process::exit(2);
    };

    let mut window: Option<Window> = None;
    let mut window_id: Option<WindowId> = None;
    let mut pixels: Option<Pixels> = None;

    let path = PathBuf::from(p);
    let image = match image::open(&path) {
        Ok(img) => img,
        Err(err) => {
            eprintln!("Erro ao abrir a imagem: {}", err);
            return;
        }
    };

    let (width, height) = image.dimensions();
    println!("Width: {}, Height: {}", width, height);

    let rgba = image.to_rgba8().into_raw();
    if rgba.len() as u32 != width * height * 4 {
        eprintln!("Buffer RGBA inválido");
        return;
    }

    let event_loop = match EventLoop::new() {
        Ok(l) => l,
        Err(err) => {
            eprintln!("Erro ao criar loop de eventos: {}", err);
            return;
        }
    };

    let _ = event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::Resumed => {
                if window.is_none() {
                    let w = WindowBuilder::new()
                        .with_title("Simple image viewer")
                        .with_inner_size(LogicalSize::new(width as f64, height as f64))
                        .build(elwt)
                        .unwrap();

                    window_id = Some(w.id());
                    window = Some(w);
                }

                let win = window.as_ref().unwrap();
                if pixels.is_none() {
                    let size = win.inner_size();
                    let surface = SurfaceTexture::new(size.width, size.height, win);
                    let p = Pixels::new(width, height, surface).unwrap();
                    pixels = Some(p);
                }

                win.request_redraw();
            }

            Event::WindowEvent {
                window_id: eid,
                event,
                ..
            } => {
                if Some(eid) != window_id {
                    return;
                }

                match event {
                    WindowEvent::CloseRequested => elwt.exit(),

                    WindowEvent::Resized(new_size) => {
                        if let Some(p) = pixels.as_mut() {
                            let _ = p.resize_surface(new_size.width, new_size.height);
                        }
                        if let Some(w) = window.as_ref() {
                            w.request_redraw();
                        }
                    }

                    WindowEvent::RedrawRequested => {
                        if let Some(p) = pixels.as_mut() {
                            p.frame_mut().copy_from_slice(&rgba);
                            p.render().unwrap();
                        }
                    }

                    _ => {}
                }
            }

            _ => {}
        }
    });
}
