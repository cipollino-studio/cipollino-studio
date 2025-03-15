
use std::sync::Arc;

use super::UI;

pub struct RedrawSignal {
    window: Arc<Box<dyn winit::window::Window>>
}

impl UI<'_, '_> {

    pub fn redraw_signal(&self) -> RedrawSignal {
        RedrawSignal {
            window: self.render_resources.window.clone()
        }
    }

}

impl RedrawSignal {

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

}