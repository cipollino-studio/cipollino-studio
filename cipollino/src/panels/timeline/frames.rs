
use egui::ScrollArea;

use crate::app::{editor::EditorState, AppSystems};

use super::{Timeline, TimelineRenderInfo};

impl Timeline {

    fn render_frames_content(&mut self, ui: &mut egui::Ui, state: &EditorState, systems: &mut AppSystems, info: &mut TimelineRenderInfo, rect: &egui::Rect, resp: &egui::Response) {
        // Render frame highlights 
        let bg_highlight_color = ui.visuals().faint_bg_color;
        for i in (0..info.len).skip(info.frame_number_step - 1).step_by(info.frame_number_step) {
            let frame_highlight_rect = egui::Rect::from_min_size(
                rect.min + egui::vec2((i as f32) * info.frame_w, 0.0),
                egui::vec2(info.frame_w, rect.height()) 
            );
            ui.painter().rect_filled(frame_highlight_rect, egui::Rounding::ZERO, bg_highlight_color);
        }

    }

    pub(super) fn render_frames(&mut self, ui: &mut egui::Ui, state: &EditorState, systems: &mut AppSystems, info: &mut TimelineRenderInfo) {
        let scroll_resp = ScrollArea::both()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
            .scroll_offset(egui::vec2(info.x_scroll, info.y_scroll)) // Always scroll to the current scroll x & y to sync with other scroll areas
            .show(ui, |ui| {
                let (rect, resp) = ui.allocate_exact_size(
                    egui::vec2(
                        (info.len as f32) * info.frame_w, 
                        ((info.layers.len() as f32) * info.layer_h).max(ui.available_height())
                    ), 
                    egui::Sense::click_and_drag()
                );

                self.render_frames_content(ui, state, systems, info, &rect, &resp);
            });
        
        // If the user hovered over the frame scroll area, update the scroll x & y based on the scroll area
        if ui.input(|i| i.pointer.hover_pos().map(|pos| scroll_resp.inner_rect.contains(pos)).unwrap_or(false)) {
            info.set_x_scroll = Some(scroll_resp.state.offset.x);
            info.set_y_scroll = Some(scroll_resp.state.offset.y);
        }
    }



}
