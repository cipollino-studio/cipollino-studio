
use project::{Ptr, Stroke};

use crate::EditorState;

use super::{Tool, ToolContext};

#[derive(Default)]
pub struct ColorPicker {

}

impl Tool for ColorPicker {

    const ICON: &'static str = pierro::icons::EYEDROPPER;
    const SHORTCUT: pierro::KeyboardShortcut = pierro::KeyboardShortcut::new(pierro::KeyModifiers::empty(), pierro::Key::P);

    fn mouse_clicked(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, _pos: elic::Vec2) {
        if let Some((x, y)) = ctx.picking_mouse_pos {
            let id = ctx.picking_buffer.read_pixel(ctx.device, ctx.queue, x, y);
            let ptr = Ptr::<Stroke>::from_key(id as u64);
            if let Some(stroke) = ctx.project.client.get(ptr) {
                editor.color = stroke.color.into();
            } 
        }
    }

}
