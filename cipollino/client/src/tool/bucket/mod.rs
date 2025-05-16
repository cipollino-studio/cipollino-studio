
use project::{Action, CreateFill, FillPaths, FillTreeData};

use crate::{curve_fit, keyboard_shortcut, EditorState}; 

use super::{Tool, ToolContext};

mod floodfill;

#[derive(Default)]
pub struct BucketTool {
    pts: Vec<elic::Vec2>,
    drawing_fill: bool,
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

    fn calc_paths(&self) -> malvina::FillPaths {
        malvina::FillPaths {
            paths: vec![calc_path(&self.pts, 1.0)] 
        }
    } 

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

    fn add_point(&mut self, pt: elic::Vec2) {
        let Some(last) = self.pts.last() else {
            self.pts.push(pt);
            return;
        };
        if last.distance(pt) > 0.5 {
            self.pts.push(pt);
        }
    }  

}

keyboard_shortcut!(BucketToolShortcut, B, pierro::KeyModifiers::empty());

impl Tool for BucketTool {

    const ICON: &'static str = pierro::icons::PAINT_BUCKET;

    type Shortcut = BucketToolShortcut;

    fn tick(&mut self, editor: &mut EditorState, _ctx: &mut ToolContext) {
        // If the user undo/redoes while drawing a fill, reset the bukcet tool
        if (editor.will_undo || editor.will_redo) && !self.pts.is_empty() {
            editor.will_undo = false;
            self.pts.clear();
            self.drawing_fill = false;
        }
        
        if self.drawing_fill {
            editor.preview.keep_preview = true;
        }
    }

    fn mouse_clicked(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: elic::Vec2) {
        if let Some((x, y)) = ctx.picking_mouse_pos {
            if ctx.pick(x, y).is_some() {
                return;
            }
        }
        floodfill::floodfill(editor, ctx, pos);
    }

    fn mouse_drag_started(&mut self, editor: &mut EditorState, _ctx: &mut ToolContext, pos: elic::Vec2) {
        if editor.can_modify_layer(editor.active_layer) {
            self.pts.clear();
            self.add_point(pos); 
            self.drawing_fill = true;
        }
    }

    fn mouse_dragged(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: elic::Vec2) {
        if !self.drawing_fill {
            return;
        } 

        self.add_point(pos);

        let paths = self.calc_paths();
        editor.preview.fill_preview = Some(malvina::FillMesh::new(ctx.device, &paths));
    }

    fn mouse_released(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: elic::Vec2) {
        if self.pts.is_empty() || !self.drawing_fill {
            return;
        } 

        self.add_point(pos);

        let paths = self.calc_paths();
        Self::create_fill(editor, ctx, paths);

        self.pts.clear();
        self.drawing_fill = false;
    }

    fn cursor_icon(&self, _editor: &mut EditorState, _ctx: &mut ToolContext, _pos: elic::Vec2) -> pierro::CursorIcon {
        pierro::CursorIcon::Crosshair
    }

}
