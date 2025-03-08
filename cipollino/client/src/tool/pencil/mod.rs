
use project::{Action, CreateStroke, StrokeData, StrokeTreeData};

use super::{Tool, ToolContext};

mod curve_fit;

#[derive(Default)]
pub struct Pencil {
    pts: Vec<malvina::StrokePoint>
}

impl Pencil {

    fn calc_stroke(&self) -> malvina::Stroke {
        let mut pts = Vec::new();
        for pt in &self.pts {
            pts.push(pt.pt.x);
            pts.push(pt.pt.y);
            pts.push(pt.pressure);
        }

        let curve_pts = curve_fit::fit_curve(3, &pts, 1.0);
        let mut stroke_pts = Vec::new();
        for i in 0..(curve_pts.len() / (3 * 3)) {
            let prev = malvina::StrokePoint::new(malvina::vec2(curve_pts[i * 9 + 0], curve_pts[i * 9 + 1]), curve_pts[i * 9 + 2]);
            let pt   = malvina::StrokePoint::new(malvina::vec2(curve_pts[i * 9 + 3], curve_pts[i * 9 + 4]), curve_pts[i * 9 + 5]);
            let next = malvina::StrokePoint::new(malvina::vec2(curve_pts[i * 9 + 6], curve_pts[i * 9 + 7]), curve_pts[i * 9 + 8]);
            stroke_pts.push(malvina::BezierPoint { prev, pt, next });
        }

        malvina::Stroke {
            path: malvina::BezierPath {
                pts: stroke_pts
            }
        }
    }

    fn create_stroke(ctx: &ToolContext, stroke: malvina::Stroke) {
        let mut action = Action::new();
        let Some(ptr) = ctx.project.client.next_ptr() else { return; };
        let Some(frame) = ctx.active_frame(&mut action) else { return; };
        action.push(CreateStroke {
            ptr,
            parent: frame,
            idx: 0,
            data: StrokeTreeData {
                stroke: StrokeData(stroke),
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

impl Tool for Pencil {
    const ICON: &'static str = pierro::icons::PENCIL;

    fn mouse_clicked(&mut self, ctx: &mut ToolContext, pos: malvina::Vec2) {
        let stroke = malvina::Stroke::point(pos, 1.0);
        Self::create_stroke(ctx, stroke); 
    }

    fn mouse_drag_started(&mut self, _ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.pts.clear();
        self.add_point(malvina::StrokePoint {
            pt: pos,
            pressure: 1.0,
        });
    }

    fn mouse_dragged(&mut self, ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.add_point(malvina::StrokePoint {
            pt: pos,
            pressure: 1.0,
        });

        let stroke = self.calc_stroke();
        ctx.editor.stroke_preview = Some(malvina::StrokeMesh::new(ctx.device, &stroke));
    }

    fn mouse_released(&mut self, ctx: &mut ToolContext, pos: malvina::Vec2) {
        ctx.clear_stroke_preview = true;
        if self.pts.is_empty() {
            return;
        }

        self.add_point(malvina::StrokePoint {
            pt: pos,
            pressure: 1.0,
        });

        let stroke = self.calc_stroke();
        self.pts.clear();
        Self::create_stroke(ctx, stroke); 
    }

    fn cursor_icon(&self) -> pierro::CursorIcon {
        pierro::CursorIcon::Crosshair
    }

}
