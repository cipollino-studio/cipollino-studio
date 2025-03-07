
use super::{FrameArea, DragState};
use crate::{panels::timeline::{render_list::RenderLayerKind, RenderList}, ProjectState, TimelinePanel};
use project::Layer;

impl FrameArea {

    fn box_select_layer(&mut self, project: &ProjectState, layer: &Layer, x_range: pierro::Range) {
        for frame_ptr in layer.frames.iter() {
            if let Some(frame) = project.client.get(frame_ptr.ptr()) {
                let frame_x_range = pierro::Range::min_size((frame.time as f32) * TimelinePanel::FRAME_WIDTH, TimelinePanel::FRAME_WIDTH);
                if frame_x_range.intersects(x_range) {
                    self.selection.select_frame(frame_ptr.ptr());
                }
            }
        }
    }

    fn box_select(&mut self, project: &ProjectState, render_list: &RenderList, from: pierro::Vec2, to: pierro::Vec2) {
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
                RenderLayerKind::Layer(_layer_ptr, layer) => self.box_select_layer(project, layer, x_range)
            }
        }
    }

    pub(super) fn drag_stopped(&mut self, project: &ProjectState, render_list: &RenderList) {
        match std::mem::replace(&mut self.drag_state, DragState::None) {
            DragState::None => {},
            DragState::Move { offset } => {
                self.selection.move_selected(project, offset);
            },
            DragState::BoxSelect { from, to } => {
                self.box_select(project, render_list, from, to);  
            },
        }
    }
    
}
