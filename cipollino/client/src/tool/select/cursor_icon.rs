
use project::{Ptr, Stroke};

use crate::ToolContext;

use super::{gizmos::PotentialDragState, DragState, SelectTool};

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
            },
            DragState::Rotate { .. } => {
                return pierro::CursorIcon::Alias;
            },
            _ => {}
        }
        
        let gizmos = self.calc_gizmos(
            ctx.key_modifiers.contains(pierro::KeyModifiers::SHIFT),
            ctx.key_modifiers.contains(pierro::KeyModifiers::OPTION),
            ctx.cam_zoom
        );
        if let Some(gizmos) = gizmos {
            match gizmos.get_pivot(pos, ctx.cam_zoom) {
                PotentialDragState::None => {},
                PotentialDragState::Scale(pivot) => {
                    return Self::diagonal_scale_cursor_icon(pivot, pos);
                },
                PotentialDragState::Rotate(_) => {
                    return pierro::CursorIcon::Alias;
                },
                PotentialDragState::Pivot => {
                    return pierro::CursorIcon::Default;
                }
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
