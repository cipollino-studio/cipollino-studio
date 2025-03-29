
use project::{Ptr, Stroke};

use crate::ToolContext;

use super::{DragState, SelectTool};

impl SelectTool {

    fn diagonal_scale_cursor_icon(pivot: elic::Vec2, mouse_pos: elic::Vec2) -> pierro::CursorIcon {
        let delta = (mouse_pos - pivot).normalize();
        if delta.x * delta.y < 0.0 {
            pierro::CursorIcon::NwseResize
        } else {
            pierro::CursorIcon::NeswResize
        }
    }

    pub(super) fn cursor_icon(&self, ctx: &mut ToolContext, pos: elic::Vec2) -> pierro::CursorIcon {
        match self.drag_state {
            DragState::Lasso(_) => {
                return pierro::CursorIcon::Default;
            },
            DragState::Move(_) => {
                return pierro::CursorIcon::Move;
            },
            DragState::Scale { pivot, .. } => {
                return Self::diagonal_scale_cursor_icon(pivot, pos);
            } 
            _ => {}
        }
        
        if let Some(gizmos) = self.calc_gizmos(ctx.key_modifiers.contains(pierro::KeyModifiers::SHIFT)) {
            if let Some(pivot) = gizmos.get_resizing_pivot(pos, ctx.cam_zoom) {
                return Self::diagonal_scale_cursor_icon(pivot, pos);
            } 
        }

        if let Some((x, y)) = ctx.picking_mouse_pos {
            let id = ctx.picking_buffer.read_pixel(ctx.device, ctx.queue, x, y);
            let ptr = Ptr::<Stroke>::from_key(id as u64);
            if !ptr.is_null() {
                return pierro::CursorIcon::Move;
            }
        }

        pierro::CursorIcon::Default
    }

}
