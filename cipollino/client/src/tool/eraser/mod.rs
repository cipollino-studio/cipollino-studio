
use project::{Action, Client, DeleteFill, DeleteStroke, SceneObjPtr};

use crate::{keyboard_shortcut, EditorState};

use super::{Tool, ToolContext};

pub struct EraserTool {
    to_delete: Vec<SceneObjPtr>,
    prev_pt: elic::Vec2
}

impl Default for EraserTool {

    fn default() -> Self {
        Self {
            to_delete: Vec::new(),
            prev_pt: elic::Vec2::ZERO
        }
    }

}

keyboard_shortcut!(EraserToolShortcut, E, pierro::KeyModifiers::empty());

impl EraserTool {

    fn delete_obj(action: &mut Action, obj: SceneObjPtr) {
        match obj {
            SceneObjPtr::Stroke(ptr) => {
                action.push(DeleteStroke {
                    ptr
                });
            },
            SceneObjPtr::Fill(ptr) => {
                action.push(DeleteFill {
                    ptr
                });
            }
        }
    }

    fn should_erase(&self, obj: SceneObjPtr, client: &Client, pos: elic::Vec2) -> bool {
        let line = elic::Segment::new(self.prev_pt, pos);

        match obj {
            SceneObjPtr::Stroke(ptr) => {
                let Some(stroke) = client.get(ptr) else { return true; };
                for stroke_segment in stroke.stroke.0.path.iter_segments() {
                    let stroke_segment = stroke_segment.map(|pt| pt.pt);
                    let ts = line.intersect_bezier_ts(&stroke_segment);
                    if !ts.is_empty() {
                        return true;
                    }
                }
            },
            SceneObjPtr::Fill(ptr) => {
                let Some(fill) = client.get(ptr) else { return true; };
                for path in &fill.paths.0.paths {
                    for path_segment in path.iter_segments() {
                        let ts = line.intersect_bezier_ts(&path_segment);
                        if !ts.is_empty() {
                            return true;
                        }
                    }
                }
            },
        }
        false
    }

}

impl Tool for EraserTool {
    const ICON: &'static str = pierro::icons::ERASER;
    type Shortcut = EraserToolShortcut;

    fn mouse_clicked(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, _pos: elic::Vec2) {
        if let Some((x, y)) = ctx.picking_mouse_pos {
            let Some(scene_obj) = ctx.pick(x, y) else { return; };
            if !ctx.modifiable_objs.contains(&scene_obj) {
                return;
            }
            let mut action = Action::new(editor.action_context("Delete")); 
            Self::delete_obj(&mut action, scene_obj);
            ctx.project.client.queue_action(action);
        }
    }

    fn tick(&mut self, editor: &mut EditorState, _ctx: &mut ToolContext) {
        if !self.to_delete.is_empty() {
            editor.preview.keep_preview = true;
        }
    }

    fn mouse_drag_started(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: elic::Vec2) {
        self.to_delete.clear(); 
        self.prev_pt = pos;
        if let Some((x, y)) = ctx.picking_mouse_pos {
            let Some(scene_obj) = ctx.pick(x, y) else { return; };
            if !ctx.modifiable_objs.contains(&scene_obj) {
                return;
            }
            self.to_delete.push(scene_obj);
            editor.preview.hide.insert(scene_obj);
        }
    }

    fn mouse_dragged(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: elic::Vec2) {
        for obj in ctx.modifiable_objs {
            if self.should_erase(*obj, &ctx.project.client, pos) {
                self.to_delete.push(*obj);
                editor.preview.hide.insert(*obj);
            }
        }

        self.prev_pt = pos;
    }

    fn mouse_drag_stopped(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, _pos: elic::Vec2) {
        let mut action = Action::new(editor.action_context("Delete"));
        for obj in &self.to_delete {
            Self::delete_obj(&mut action, *obj);
        }
        ctx.project.client.queue_action(action);
        self.to_delete.clear();
    }

}
