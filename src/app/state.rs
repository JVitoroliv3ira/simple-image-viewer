use pixels::Pixels;
use winit::window::{Window, WindowId};

pub struct AppState {
    pub window: Option<Window>,
    pub window_id: Option<WindowId>,
    pub pixels: Option<Pixels>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            window: None,
            window_id: None,
            pixels: None,
        }
    }

    pub fn is_siv_window(&self, id: WindowId) -> bool {
        self.window_id == Some(id)
    }
}
