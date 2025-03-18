
use project::Layer;

use crate::{EditorState, ProjectState, TimelinePanel};

use super::{DragState, FrameArea, PaintCommands};

pub(super) struct FrameDot {
    pub layer_idx: usize, 
    pub time: i32,
    pub selected: bool
}

impl FrameDot {

    pub fn paint(self, painter: &mut pierro::Painter, rect: pierro::Rect, text_color: pierro::Color, accent_color: pierro::Color) {
        let frame_rect = pierro::Rect::min_size(
            rect.tl() + TimelinePanel::FRAME_SIZE * pierro::vec2(self.time as f32, self.layer_idx as f32),
            TimelinePanel::FRAME_SIZE 
        );
        if self.selected {
            painter.rect(
                pierro::PaintRect::new(frame_rect, pierro::Color::TRANSPARENT)
                    .with_stroke(pierro::Stroke::new(accent_color, 1.5))
                    .with_rounding(pierro::Rounding::same(3.0))
            );
        }

        let frame_dot_rect = pierro::Rect::center_size(frame_rect.center(), pierro::Vec2::splat(TimelinePanel::FRAME_WIDTH * 0.6));
        painter.rect(
            pierro::PaintRect::new(frame_dot_rect, text_color)
                .with_rounding(pierro::Rounding::same(frame_dot_rect.width() / 1.5))
        );
    }

}

impl FrameArea {
    
    pub(super) fn drag_to_frame_offset(drag: f32) -> i32 {
        (drag / TimelinePanel::FRAME_WIDTH).round() as i32
    }

    pub(super) fn render_layer_contents(
        &mut self,
        ui: &mut pierro::UI,
        project: &ProjectState,
        editor: &mut EditorState,
        frame_area: &pierro::Response,
        paint_commands: &mut PaintCommands,
        layer_idx: usize,
        layer: &Layer
    ) {

        for frame_ptr in layer.frames.iter() {
            if let Some(frame) = project.client.get(frame_ptr.ptr()) {

                let frame_interaction_rect = pierro::Rect::min_size(
                    pierro::vec2((frame.time as f32) * TimelinePanel::FRAME_WIDTH, (layer_idx as f32) * TimelinePanel::LAYER_HEIGHT),
                    TimelinePanel::FRAME_SIZE 
                );

                let selected = editor.selection.selected(frame_ptr.ptr());
                
                let in_selection_rect = if let Some(selection_rect) = self.drag_state.selection_rect() {
                    selection_rect.intersects(frame_interaction_rect)
                } else {
                    false
                };

                let display_time = frame.time + if selected {
                    Self::drag_to_frame_offset(self.drag_state.move_offset())
                } else {
                    0
                };
                let display_time = display_time.max(0);

                paint_commands.frame_dots.push(FrameDot {
                    layer_idx,
                    time: display_time,
                    selected: selected || in_selection_rect
                });
                
                
                if let Some(mouse_pos) = frame_area.mouse_pos(ui) {
                    
                    if frame_interaction_rect.contains(mouse_pos) {
                        if frame_area.mouse_clicked() {
                            editor.selection.extend_select(frame_ptr.ptr());
                            frame_area.request_focus(ui);
                        }
                        if frame_area.drag_started() {
                            if !editor.selection.selected(frame_ptr.ptr()) && !ui.input().key_down(&pierro::Key::Backspace) {
                                editor.selection.clear();
                            }
                            editor.selection.select(frame_ptr.ptr());
                            self.drag_consumed = true;
                            self.drag_state = DragState::Move { offset: 0.0 };
                            frame_area.request_focus(ui);
                        }
                    }
                }
                
            }
        }

    }

}
