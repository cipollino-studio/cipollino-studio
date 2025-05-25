
use project::SceneObjPtr;

use crate::{keyboard_shortcut, EditorState};

use super::{Tool, ToolContext};

#[derive(Default)]
pub struct ColorPicker {

}

keyboard_shortcut!(ColorPickerShortcut, P, pierro::KeyModifiers::empty());

impl Tool for ColorPicker {

    const ICON: &'static str = pierro::icons::EYEDROPPER;

    type Shortcut = ColorPickerShortcut;

    fn mouse_clicked(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, _pos: elic::Vec2) {
        if let Some((x, y)) = ctx.picking_mouse_pos {
            match ctx.pick(x, y) {
                Some(SceneObjPtr::Stroke(ptr)) => {
                    if let Some(stroke) = ctx.project.client.get(ptr) {
                        editor.color = stroke.color;
                    } 
                },
                Some(SceneObjPtr::Fill(ptr)) => {
                    if let Some(fill) = ctx.project.client.get(ptr) {
                        editor.color = fill.color;
                    }
                }
                None => {},
            }
        }
    }

}
