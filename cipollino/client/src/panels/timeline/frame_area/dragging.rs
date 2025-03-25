
use super::{FrameArea, DragState};
use crate::{panels::timeline::{render_list::RenderLayerKind, RenderList}, EditorState, ProjectState, TimelinePanel};
use project::{Action, Frame, Layer, SetFrameTime};

impl FrameArea {

    fn box_select_layer(&mut self, project: &ProjectState, editor: &mut EditorState, layer: &Layer, x_range: pierro::Range) {
        for frame_ptr in layer.frames.iter() {
            if let Some(frame) = project.client.get(frame_ptr.ptr()) {
                let frame_x_range = pierro::Range::min_size((frame.time as f32) * TimelinePanel::FRAME_WIDTH, TimelinePanel::FRAME_WIDTH);
                if frame_x_range.intersects(x_range) {
                    editor.selection.select(frame_ptr.ptr());
                }
            }
        }
    }

    fn box_select(&mut self, project: &ProjectState, editor: &mut EditorState, render_list: &RenderList, from: pierro::Vec2, to: pierro::Vec2) {
        let min = from.min(to);
        let max = from.max(to);
        
        let x_range = pierro::Range::new(min.x, max.x);
        let top_layer_idx = ((min.y / TimelinePanel::LAYER_HEIGHT).floor() as i32).max(0);
        let bottom_layer_idx = ((max.y / TimelinePanel::LAYER_HEIGHT).floor() as i32).max(0);
        for layer_idx in top_layer_idx..=bottom_layer_idx {
            if layer_idx as usize >= render_list.len() {
                break;
            }
            let layer = &render_list.layers[layer_idx as usize];
            match &layer.kind {
                RenderLayerKind::Layer(_layer_ptr, layer) => self.box_select_layer(project, editor, layer, x_range)
            }
        }
    }

    fn move_selected(project: &ProjectState, editor: &EditorState, drag: f32) {
        let mut action = Action::new(editor.action_context("Move Frames"));

        let frame_offset = FrameArea::drag_to_frame_offset(drag);

        let mut selected_frames = Vec::new();
        for frame_ptr in editor.selection.iter() {
            if let Some(frame) = project.client.get::<Frame>(frame_ptr) {
                selected_frames.push((frame.time, frame_ptr));
            }
        }
        selected_frames.sort_by_key(|(time, _)| *time);
        if frame_offset > 0 {
            selected_frames.reverse();
        }
        for (time, frame) in selected_frames {
            action.push(SetFrameTime {
                frame,
                new_time: time + frame_offset,
                ..Default::default()
            });
        } 
         
        project.client.queue_action(action);

    }

    pub(super) fn drag_stopped(&mut self, project: &ProjectState, editor: &mut EditorState, render_list: &RenderList) {
        match std::mem::replace(&mut self.drag_state, DragState::None) {
            DragState::None => {},
            DragState::Move { offset } => {
                Self::move_selected(project, editor, offset);
            },
            DragState::BoxSelect { from, to } => {
                self.box_select(project, editor, render_list, from, to);  
            },
        }
    }
    
}
