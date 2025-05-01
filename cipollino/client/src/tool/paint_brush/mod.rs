
use project::{Action, SceneObjPtr, SetFillColor, SetStrokeColor};

use crate::{keyboard_shortcut, EditorState};

use super::{LassoState, Tool, ToolContext};

#[derive(Default)]
pub struct PaintBrushTool {
    lasso: Option<LassoState>
}

keyboard_shortcut!(PaintBrushToolShortcut, Q, pierro::KeyModifiers::empty());

impl Tool for PaintBrushTool {

    const ICON: &'static str = pierro::icons::PAINT_BRUSH;

    type Shortcut = PaintBrushToolShortcut;

    fn mouse_clicked(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, _pos: malvina::Vec2) {
        if let Some((x, y)) = ctx.picking_mouse_pos {
            match ctx.pick(x, y) {
                Some(SceneObjPtr::Stroke(ptr)) => {
                    if ctx.modifiable_objs.contains(&ptr.into()) {
                        ctx.project.client.queue_action(Action::single(editor.action_context("Set Stroke Color"), SetStrokeColor {
                            ptr,
                            color_value: editor.color.into(),
                        }));
                    }
                },
                Some(SceneObjPtr::Fill(ptr)) => {
                    if ctx.modifiable_objs.contains(&ptr.into()) {
                        ctx.project.client.queue_action(Action::single(editor.action_context("Set Fill Color"), SetFillColor {
                            ptr,
                            color_value: editor.color.into(),
                        }));
                    }
                }
                None => {},
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
            let mut action = Action::new(editor.action_context("Set Color"));
            for scene_obj in lasso.find_inside(&ctx.project.client, ctx.modifiable_objs) {
                match scene_obj {
                    SceneObjPtr::Stroke(stroke) => {
                        action.push(SetStrokeColor {
                            ptr: stroke,
                            color_value: editor.color.into(),
                        });
                    },
                    SceneObjPtr::Fill(fill) => {
                        action.push(SetFillColor {
                            ptr: fill,
                            color_value: editor.color.into(),
                        });
                    },
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
