
use super::layer::FrameDot;

/// Commands for painting frame dots, audio clips, etc in the timeline's frame area.
/// This is necessary because painting happens after the UI tree is constructed,
/// so we can't use any borrowed data in the paint callback. Using a command
/// queue gets around this problem.
pub(super) struct PaintCommands {
    pub frame_dots: Vec<FrameDot>
}

impl PaintCommands {

    pub fn new() -> Self {
        Self {
            frame_dots: Vec::new()
        }
    }

    pub fn paint(self, painter: &mut pierro::Painter, rect: pierro::Rect, text_color: pierro::Color) {
        for frame_dot in self.frame_dots {
            frame_dot.paint(painter, rect, text_color);
        }
    }

}
