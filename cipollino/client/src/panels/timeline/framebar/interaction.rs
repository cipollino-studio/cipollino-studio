
use project::ClipInner;

use crate::{EditorState, TimelinePanel};

use super::{DragTarget, Framebar};

impl Framebar {

    pub(super) fn mouse_interaction(&mut self, ui: &mut pierro::UI, framebar_response: &pierro::Response, editor: &mut EditorState, clip: &ClipInner) {
        ui.set_sense_mouse(framebar_response.node_ref, true);
        let curr_frame = clip.frame_idx(editor.time);

        // Figure out what we're hovering over 
        let hovered_drag_target = if editor.show_onion_skin { 
            let prev_onion_skin_frame = (curr_frame - (editor.onion_skin_prev_frames as i32)).max(0);
            let onion_skin_prev_x_range = pierro::Range::center_size(
                (prev_onion_skin_frame as f32) * TimelinePanel::FRAME_WIDTH,
                TimelinePanel::FRAME_WIDTH * 1.5
            );
            let next_onion_skin_frame = (curr_frame + (editor.onion_skin_next_frames as i32)).max(0);
            let onion_skin_next_x_range = pierro::Range::center_size(
                (next_onion_skin_frame as f32) * TimelinePanel::FRAME_WIDTH + TimelinePanel::FRAME_WIDTH,
                TimelinePanel::FRAME_WIDTH * 1.5
            );
            match framebar_response.mouse_pos(ui) {
                Some(pos) => match pos.x {
                    x if onion_skin_prev_x_range.contains(x) => DragTarget::OnionSkinPrev,
                    x if onion_skin_next_x_range.contains(x) => DragTarget::OnionSkinNext,
                    _ => DragTarget::PlayHead
                },
                None => DragTarget::PlayHead
            }
        } else {
            DragTarget::PlayHead
        };

        // Set the mouse cursor according to what we're hovering
        if framebar_response.hovered {
            let drag_target = if framebar_response.is_focused(ui) {
                self.drag_target
            } else {
                hovered_drag_target
            };
            ui.set_cursor(match drag_target {
                DragTarget::PlayHead => pierro::CursorIcon::Default,
                DragTarget::OnionSkinPrev | DragTarget::OnionSkinNext => pierro::CursorIcon::EwResize,
            });
        }

        if framebar_response.drag_started() {
            framebar_response.request_focus(ui);
            self.drag_target = hovered_drag_target;
        }

        if framebar_response.is_focused(ui) && !framebar_response.dragging() {
            framebar_response.release_focus(ui);
        }
        if let Some(mouse_pos) = framebar_response.mouse_pos(ui) {
            let frame = ((mouse_pos.x / TimelinePanel::FRAME_WIDTH).floor() as i32).max(0);
            if framebar_response.mouse_clicked() {
                editor.jump_to((frame.min(clip.length as i32 - 1) as f32) * clip.frame_len() + 0.01);
            }
            if framebar_response.is_focused(ui) {
                match self.drag_target {
                    DragTarget::PlayHead => editor.jump_to((frame.min(clip.length as i32 - 1) as f32) * clip.frame_len() + 0.01),
                    DragTarget::OnionSkinPrev => editor.onion_skin_prev_frames = (curr_frame - frame).max(0) as u32,
                    DragTarget::OnionSkinNext => editor.onion_skin_next_frames = (frame - curr_frame).max(0) as u32,
                }
            }
        }
    }

}
