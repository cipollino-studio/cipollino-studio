
use project::{Ptr, Stroke};
use super::{Tool, ToolContext};

#[derive(Default)]
pub struct SelectTool {

}

impl Tool for SelectTool {

    const ICON: &'static str = pierro::icons::CURSOR;

    fn mouse_dragged(&mut self, _ctx: &mut ToolContext, _pos: malvina::Vec2) {
        
    }

    fn mouse_clicked(&mut self, ctx: &mut ToolContext, _pos: malvina::Vec2) {
        if let Some((x, y)) = ctx.picking_mouse_pos {
            let id = ctx.picking_buffer.read_pixel(ctx.device, ctx.queue, x, y);
            let ptr = Ptr::<Stroke>::from_key(id as u64);
            ctx.editor.selection.extend_select(ptr);
        } 
    }

}
