
use project::Layer;

use crate::{ProjectState, TimelinePanel};

use super::PaintCommands;

pub(super) struct FrameDot {
    pub layer_idx: usize, 
    pub time: i32,
}

impl FrameDot {

    pub fn paint(self, painter: &mut pierro::Painter, rect: pierro::Rect, text_color: pierro::Color) {
        let frame_rect = pierro::Rect::min_size(
            rect.tl() + TimelinePanel::FRAME_SIZE * pierro::vec2(self.time as f32, self.layer_idx as f32),
            TimelinePanel::FRAME_SIZE 
        );

        let frame_dot_rect = pierro::Rect::center_size(frame_rect.center(), pierro::Vec2::splat(TimelinePanel::FRAME_WIDTH * 0.6));
        painter.rect(
            pierro::PaintRect::new(frame_dot_rect, text_color)
                .with_rounding(pierro::Rounding::same(frame_dot_rect.width() / 1.5))
        );
    }

}

impl TimelinePanel {

    pub(super) fn render_layer_contents(&mut self, project: &ProjectState, paint_commands: &mut PaintCommands, layer_idx: usize, layer: &Layer) {

        for frame in layer.frames.iter() {
            if let Some(frame) = project.client.get(frame) {
                paint_commands.frame_dots.push(FrameDot {
                    layer_idx,
                    time: frame.time,
                });
            }
        }

    }

}