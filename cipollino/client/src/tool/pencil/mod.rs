
use project::{Action, CreateStroke, StrokeData, StrokeTreeData};

use super::{Tool, ToolContext};

mod curve_fit;

#[derive(Default)]
pub struct PencilTool {
    pts: Vec<malvina::StrokePoint>,
    drawing_stroke: bool,
}

impl PencilTool {

    fn calc_stroke(&self) -> malvina::Stroke {
        let mut pts = Vec::new();
        let pressure_sensitivity = 5.0;
        for pt in &self.pts {
            pts.push(pt.pt.x);
            pts.push(pt.pt.y);
            pts.push(pt.pressure * pressure_sensitivity);
        }

        let curve_pts = curve_fit::fit_curve(3, &pts, 1.0);
        let mut stroke_pts = Vec::new();
        for i in 0..(curve_pts.len() / (3 * 3)) {
            let prev = malvina::StrokePoint::new(malvina::vec2(curve_pts[i * 9 + 0], curve_pts[i * 9 + 1]), curve_pts[i * 9 + 2] / pressure_sensitivity);
            let pt   = malvina::StrokePoint::new(malvina::vec2(curve_pts[i * 9 + 3], curve_pts[i * 9 + 4]), curve_pts[i * 9 + 5] / pressure_sensitivity);
            let next = malvina::StrokePoint::new(malvina::vec2(curve_pts[i * 9 + 6], curve_pts[i * 9 + 7]), curve_pts[i * 9 + 8] / pressure_sensitivity);
            stroke_pts.push(elic::BezierPoint { prev, pt, next });
        }

        malvina::Stroke {
            path: elic::BezierPath {
                pts: stroke_pts
            }
        }
    }

    fn create_stroke(ctx: &ToolContext, stroke: malvina::Stroke) {
        let mut action = Action::new(ctx.editor.action_context("New Stroke"));
        let Some(ptr) = ctx.project.client.next_ptr() else { return; };
        let Some(frame) = ctx.active_frame(&mut action) else { return; };
        action.push(CreateStroke {
            ptr,
            parent: frame,
            idx: 0,
            data: StrokeTreeData {
                stroke: StrokeData(stroke),
                color: ctx.editor.color.into()
            },
        });
        ctx.project.client.queue_action(action);
    }

    fn add_point(&mut self, pt: malvina::StrokePoint) {
        let Some(last) = self.pts.last() else {
            self.pts.push(pt);
            return;
        };
        if last.pt.distance(pt.pt) > 0.5 {
            self.pts.push(pt);
        }
    }

}

impl Tool for PencilTool {

    const ICON: &'static str = pierro::icons::PENCIL;
    const SHORTCUT: pierro::KeyboardShortcut = pierro::KeyboardShortcut::new(
        pierro::KeyModifiers::empty(),
        pierro::Key::D
    );

    fn tick(&mut self, ctx: &mut ToolContext) {
        // If the user undo/redoes while drawing as stroke, reset the pencil tool
        if (ctx.editor.will_undo || ctx.editor.will_redo) && !self.pts.is_empty() {
            ctx.editor.will_undo = false;
            self.pts.clear();
            self.drawing_stroke = false;
        }

        if self.drawing_stroke {
            ctx.editor.preview.keep_preview = true;
        }
    }

    fn mouse_clicked(&mut self, ctx: &mut ToolContext, pos: malvina::Vec2) {
        let stroke = malvina::Stroke::point(pos, 1.0);
        Self::create_stroke(ctx, stroke); 
    }

    fn mouse_drag_started(&mut self, ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.pts.clear();
        self.add_point(malvina::StrokePoint {
            pt: pos,
            pressure: ctx.pressure,
        });
        self.drawing_stroke = true;
    }

    fn mouse_dragged(&mut self, ctx: &mut ToolContext, pos: malvina::Vec2) {
        if !self.drawing_stroke {
            return;
        }

        self.add_point(malvina::StrokePoint {
            pt: pos,
            pressure: ctx.pressure,
        });

        let stroke = self.calc_stroke();
        ctx.editor.preview.stroke_preview = Some(malvina::StrokeMesh::new(ctx.device, &stroke));
    }

    fn mouse_released(&mut self, ctx: &mut ToolContext, pos: malvina::Vec2) {
        if self.pts.is_empty() || !self.drawing_stroke {
            return;
        }

        self.add_point(malvina::StrokePoint {
            pt: pos,
            pressure: ctx.pressure,
        });

        let stroke = self.calc_stroke();
        self.pts.clear();
        self.drawing_stroke = false;
        Self::create_stroke(ctx, stroke); 
    }

    fn cursor_icon(&self) -> pierro::CursorIcon {
        pierro::CursorIcon::Crosshair
    }

}
