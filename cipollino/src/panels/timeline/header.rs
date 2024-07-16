
use crate::editor::EditorState;

use super::{Timeline, TimelineCommand, TimelineRenderInfo, PLAYBACK_HEAD_HIGHLIGHT_COLOR, PLAYBACK_HEAD_STROKE_COLOR};


impl Timeline {

    fn frame_rect(&self, rect: &egui::Rect, info: &TimelineRenderInfo, frame: i32) -> egui::Rect {
        let center = rect.min + egui::vec2((frame as f32 + 0.5) * info.frame_w, rect.height() / 2.0);
        egui::Rect::from_center_size(center, egui::vec2(info.frame_w, rect.height()))
    }

    fn set_playback_frame_to_mouse(&self, rect: &egui::Rect, resp: &egui::Response, info: &mut TimelineRenderInfo) {
        let Some(mouse_pos) = resp.interact_pointer_pos() else { return; };
        let mouse_x = mouse_pos.x - rect.left();
        info.commands.push(TimelineCommand::SetPlaybackFrame((mouse_x / info.frame_w).floor() as i32));
    }

    pub(super) fn render_header(&mut self, ui: &mut egui::Ui, state: &EditorState, info: &mut TimelineRenderInfo) {
        let scroll_resp = egui::ScrollArea::horizontal()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .scroll_offset(egui::vec2(info.x_scroll, 0.0)) // Always scroll to the current scroll x to sync with other scroll areas
            .show(ui, |ui| {
                let (rect, resp) = ui.allocate_exact_size(
                    egui::vec2(info.frame_w * (info.len as f32), ui.available_height()),
                    egui::Sense::click_and_drag()
                );

                // Draw frame numbers
                for i in (0..info.len).skip(info.frame_number_step - 1).step_by(info.frame_number_step) {
                    let frame_rect = self.frame_rect(&rect, info, i);
                    ui.put(frame_rect, egui::Label::new(format!("{}", i + 1)).selectable(false).wrap(false));
                }

                // Playback head
                let frame = state.playback_frame(); 
                let mut frame_rect = self.frame_rect(&rect, info, frame).shrink(0.5);
                frame_rect.max.y -= 1.0;
                ui.painter().rect(
                    frame_rect,
                    egui::Rounding::ZERO,
                    PLAYBACK_HEAD_HIGHLIGHT_COLOR,
                    egui::Stroke {
                        width: 1.0,
                        color: PLAYBACK_HEAD_STROKE_COLOR,
                    }
                );

                // Header interaction
                if resp.drag_started() {
                    resp.request_focus();
                }
                if resp.clicked() || resp.has_focus() {
                    self.set_playback_frame_to_mouse(&rect, &resp, info);
                }
                if resp.drag_stopped() {
                    resp.surrender_focus();
                }
            });

        // If the user hovered over the header scroll area, update the scroll x based on the scroll area
        if ui.input(|i| i.pointer.hover_pos().map(|pos| scroll_resp.inner_rect.contains(pos)).unwrap_or(false)) {
            info.set_x_scroll = Some(scroll_resp.state.offset.x);
        }
    }

}