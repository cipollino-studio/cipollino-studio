
use project::{Action, CreateFill, FillPaths, FillTreeData, SceneObjPtr, SetFillColor, SetStrokeColor};

use crate::{curve_fit, keyboard_shortcut, EditorState}; 

use super::{LassoState, Tool, ToolContext};

mod floodfill;

#[derive(Default)]
pub struct BucketTool {
    lasso: Option<LassoState>,
    #[cfg(debug_assertions)]
    show_collision_segments: bool
}

fn calc_path(pts: &Vec<elic::Vec2>, error: f32) -> elic::BezierPath<elic::Vec2> {
    let mut vals = Vec::new();
    for pt in pts {
        vals.push(pt.x);
        vals.push(pt.y);
    }

    let curve_pts = curve_fit::fit_curve(2, &vals, error);
    let mut stroke_pts = Vec::new();
    for i in 0..(curve_pts.len() / (2 * 3)) {
        let prev = elic::vec2(curve_pts[i * 6 + 0], curve_pts[i * 6 + 1]);
        let pt   = elic::vec2(curve_pts[i * 6 + 2], curve_pts[i * 6 + 3]);
        let next = elic::vec2(curve_pts[i * 6 + 4], curve_pts[i * 6 + 5]);
        stroke_pts.push(elic::BezierPoint { prev, pt, next });
    }

    elic::BezierPath {
        pts: stroke_pts,
    }
}

impl BucketTool {

    fn create_fill(editor: &mut EditorState, ctx: &mut ToolContext, fill: malvina::FillPaths) {
        let mut action = Action::new(editor.action_context("New Fill"));
        let ptr = ctx.project.client.next_ptr();
        let Some(frame) = ctx.active_frame(editor, &mut action) else { return; };
        let idx = ctx.project.client.get(frame).map(|frame| frame.scene.as_slice().len()).unwrap_or(0);
        action.push(CreateFill {
            ptr,
            parent: frame,
            idx,
            data: FillTreeData {
                color: editor.color.into(),
                paths: FillPaths(fill),
            },
        });
        ctx.project.client.queue_action(action);
    }

}

keyboard_shortcut!(BucketToolShortcut, B, pierro::KeyModifiers::empty());

impl Tool for BucketTool {

    const ICON: &'static str = pierro::icons::PAINT_BUCKET;

    type Shortcut = BucketToolShortcut;

    fn mouse_clicked(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: elic::Vec2) {
        if let Some((x, y)) = ctx.picking_mouse_pos {
            if let Some(obj) = ctx.pick(x, y) {
                match obj {
                    SceneObjPtr::Stroke(ptr) => {
                        ctx.project.client.queue_action(Action::single(editor.action_context("Set stroke color"), SetStrokeColor {
                            ptr,
                            color_value: editor.color.into(),
                        }));
                    },
                    SceneObjPtr::Fill(ptr) => {
                        ctx.project.client.queue_action(Action::single(editor.action_context("Set fill color"), SetFillColor {
                            ptr,
                            color_value: editor.color.into(),
                        }));
                    },
                }
                return;
            }
        }

        floodfill::floodfill(editor, ctx, pos);
    }

    fn mouse_drag_started(&mut self, _editor: &mut EditorState, _ctx: &mut ToolContext, pos: elic::Vec2) {
        self.lasso = Some(LassoState::from_point(pos));
    }

    fn mouse_dragged(&mut self, _editor: &mut EditorState, _ctx: &mut ToolContext, pos: elic::Vec2) {
        if let Some(lasso) = &mut self.lasso {
            lasso.add_point(pos);
        }
    }

    fn mouse_released(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: elic::Vec2) {
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

    fn render_overlay(&self, ctx: &mut ToolContext, rndr: &mut malvina::LayerRenderer, accent_color: elic::Color) {
        if let Some(lasso) = &self.lasso {
            lasso.render_overlay(rndr, accent_color);
        } 

        #[cfg(debug_assertions)]
        if self.show_collision_segments {
            floodfill::overlay_collision_segments(ctx, rndr);
        }
    }

    fn cursor_icon(&self, _editor: &mut EditorState, _ctx: &mut ToolContext, _pos: elic::Vec2) -> pierro::CursorIcon {
        pierro::CursorIcon::Crosshair
    }

    fn settings(&mut self, ui: &mut pierro::UI, _systems: &mut crate::AppSystems) {
        pierro::key_value_layout(ui, |builder| {
            #[cfg(debug_assertions)]
            builder.labeled("DEBUG: Show collision beziers", |ui| {
                pierro::checkbox(ui, &mut self.show_collision_segments);
            });
        }) 
    }

}
