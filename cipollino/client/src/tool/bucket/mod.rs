
use project::{Action, Ptr, SetStrokeColor, Stroke};

use crate::EditorState;

use super::{LassoState, Tool, ToolContext};

#[derive(Default)]
pub struct BucketTool {
    lasso: Option<LassoState>
}

impl Tool for BucketTool {

    const ICON: &'static str = pierro::icons::PAINT_BUCKET;
    const SHORTCUT: pierro::KeyboardShortcut = pierro::KeyboardShortcut::new(pierro::KeyModifiers::empty(), pierro::Key::B);

    fn mouse_clicked(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, _pos: malvina::Vec2) {
        if let Some((x, y)) = ctx.picking_mouse_pos {
            let id = ctx.picking_buffer.read_pixel(ctx.device, ctx.queue, x, y);
            let ptr = Ptr::<Stroke>::from_key(id as u64);
            if ctx.project.client.get(ptr).is_some() {
                ctx.project.client.queue_action(Action::single(editor.action_context("Set Stroke Color"), SetStrokeColor {
                    ptr,
                    color_value: editor.color.into(),
                }));
            } 
        }
    }

    fn mouse_drag_started(&mut self, _editor: &mut EditorState, _ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.lasso = Some(LassoState::from_point(pos));
    }

    fn mouse_dragged(&mut self, _editor: &mut EditorState, _ctx: &mut ToolContext, pos: malvina::Vec2) {
        if let Some(lasso) = &mut self.lasso {
            lasso.add_point(pos);
        }
    }

    fn mouse_drag_stopped(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: malvina::Vec2) {
        if let Some(mut lasso) = self.lasso.take() {
            lasso.add_point(pos);
            let mut action = Action::new(editor.action_context("Set Stroke Color"));
            for stroke in lasso.find_inside(&ctx.project.client, ctx.rendered_strokes) {
                if ctx.project.client.get(stroke).is_some() {
                    action.push(SetStrokeColor {
                        ptr: stroke,
                        color_value: editor.color.into(),
                    });
                }
            }
            ctx.project.client.queue_action(action);
        }
    }

    fn render_overlay(&self, _ctx: &mut ToolContext, rndr: &mut malvina::LayerRenderer, accent_color: elic::Color) {
        if let Some(lasso) = &self.lasso {
            lasso.render_overlay(rndr, accent_color);
        } 
    }

}
