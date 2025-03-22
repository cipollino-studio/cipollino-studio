
use project::{Ptr, Stroke};
use super::{Tool, ToolContext};

#[derive(Default)]
pub struct SelectTool {
    lasso_pts: Vec<malvina::Vec2>
}

impl Tool for SelectTool {

    const ICON: &'static str = pierro::icons::CURSOR;
    const SHORTCUT: pierro::KeyboardShortcut = pierro::KeyboardShortcut::new(
        pierro::KeyModifiers::empty(),
        pierro::Key::V
    );

    fn mouse_drag_started(&mut self, _ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.lasso_pts.clear();
        self.lasso_pts.push(pos); 
    }

    fn mouse_dragged(&mut self, _ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.lasso_pts.push(pos); 
    }

    fn mouse_drag_stopped(&mut self, _ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.lasso_pts.push(pos); 
        
        self.lasso_pts.clear();
    }

    fn mouse_clicked(&mut self, ctx: &mut ToolContext, _pos: malvina::Vec2) {
        if let Some((x, y)) = ctx.picking_mouse_pos {
            let id = ctx.picking_buffer.read_pixel(ctx.device, ctx.queue, x, y);
            let ptr = Ptr::<Stroke>::from_key(id as u64);
            ctx.editor.selection.extend_select(ptr);
        } 
    }

    fn render_overlay(&self, rndr: &mut malvina::LayerRenderer) {
        if self.lasso_pts.len() < 2 {
            return;
        }
        for i in 0..(self.lasso_pts.len() - 1) {
            let a = self.lasso_pts[i];
            let b = self.lasso_pts[i + 1];
            rndr.overlay_line(a, b);
        }
    }

}
