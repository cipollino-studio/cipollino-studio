
use project::{Action, CreateFill, CreateStroke, FillPaths, FillTreeData, StrokeData, StrokeTreeData};

use crate::{keyboard_shortcut, AppSystems, EditorState, UserPrefs};

use super::{curve_fit, get_active_color, Tool, ToolContext};

mod prefs;
use prefs::*;

mod settings;

pub struct PencilTool {
    pts: Vec<malvina::StrokePoint>,
    drawing_stroke: bool,
    draw_fill: bool,
    prev_mouse_pos: elic::Vec2
}

impl Default for PencilTool {

    fn default() -> Self {
        Self {
            pts: Vec::new(),
            drawing_stroke: false,
            draw_fill: false,
            prev_mouse_pos: elic::Vec2::ZERO
        }
    }

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

    fn calc_fill(&self) -> malvina::FillPaths {
        let mut pts = Vec::new();
        for pt in &self.pts {
            pts.push(pt.pt.x);
            pts.push(pt.pt.y);
        }

        let curve_pts = curve_fit::fit_curve(2, &pts, 1.0);
        let mut fill_pts = Vec::new();
        for i in 0..(curve_pts.len() / (3 * 2)) {
            let prev = malvina::vec2(curve_pts[i * 6 + 0], curve_pts[i * 6 + 1]);
            let pt   = malvina::vec2(curve_pts[i * 6 + 2], curve_pts[i * 6 + 3]);
            let next = malvina::vec2(curve_pts[i * 6 + 4], curve_pts[i * 6 + 5]);
            fill_pts.push(elic::BezierPoint { prev, pt, next });
        }

        malvina::FillPaths {
            paths: vec![elic::BezierPath { pts: fill_pts }]
        }
    } 

    fn create_stroke(editor: &mut EditorState, ctx: &mut ToolContext, stroke: malvina::Stroke) {
        let mut action = Action::new(editor.action_context("New Stroke"));
        let ptr = ctx.project.client.next_ptr();
        let Some(frame) = ctx.active_frame(editor, &mut action) else { return; };
        let stroke_width = ctx.systems.prefs.get::<PencilStrokeWidthPref>();
        let color = get_active_color(&ctx.project.client, editor, &mut action);
        action.push(CreateStroke {
            ptr,
            parent: frame,
            idx: 0,
            data: StrokeTreeData {
                stroke: StrokeData(stroke),
                color,
                width: stroke_width 
            },
        });
        ctx.project.client.queue_action(action);
    }

    fn create_fill(editor: &mut EditorState, ctx: &mut ToolContext, fill: malvina::FillPaths) {
        let mut action = Action::new(editor.action_context("New Fill"));
        let ptr = ctx.project.client.next_ptr();
        let Some(frame) = ctx.active_frame(editor, &mut action) else { return; };
        let idx = ctx.project.client.get(frame).map(|frame| frame.scene.as_slice().len()).unwrap_or(0);
        let color = get_active_color(&ctx.project.client, editor, &mut action);
        action.push(CreateFill {
            ptr,
            parent: frame,
            idx,
            data: FillTreeData {
                paths: FillPaths(fill),
                color
            } 
        });
        ctx.project.client.queue_action(action);
    }

    fn add_point(&mut self, mut pt: malvina::StrokePoint, systems: &mut AppSystems) {
        if !systems.prefs.get::<PencilUsePressure>() {
            pt.pressure = 1.0;
        }

        let Some(last) = self.pts.last() else {
            self.pts.push(pt);
            return;
        };
        if last.pt.distance(pt.pt) > 0.5 {
            self.pts.push(pt);
        }
    }

    fn stabilize_point(&self, pos: elic::Vec2, prefs: &mut UserPrefs) -> elic::Vec2 {
        let Some(prev_pt) = self.pts.last() else { return pos; };
        let prev_pt = prev_pt.pt;
        let stabilization_radius = prefs.get::<StabilizationRadius>() as f32;
        if prev_pt.distance(pos) < stabilization_radius {
            return prev_pt;
        }

        pos + (prev_pt - pos).normalize() * stabilization_radius
    }

}

keyboard_shortcut!(PencilToolShortcut, D, pierro::KeyModifiers::empty());

impl Tool for PencilTool {

    const ICON: &'static str = pierro::icons::PENCIL;

    type Shortcut = PencilToolShortcut;

    fn tick(&mut self, editor: &mut EditorState, _ctx: &mut ToolContext) {
        // If the user undo/redoes while drawing a stroke, reset the pencil tool
        if (editor.will_undo || editor.will_redo) && !self.pts.is_empty() {
            editor.will_undo = false;
            self.pts.clear();
            self.drawing_stroke = false;
        }

        if self.drawing_stroke {
            editor.preview.keep_preview = true;
        }
    }

    fn mouse_clicked(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: malvina::Vec2) {
        let stroke = malvina::Stroke::point(pos, 1.0);
        Self::create_stroke(editor, ctx, stroke); 
    }

    fn mouse_drag_started(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.prev_mouse_pos = pos;
        
        if editor.can_modify_layer(editor.active_layer) {
            self.pts.clear();
            self.add_point(malvina::StrokePoint {
                pt: pos,
                pressure: ctx.pressure,
            }, ctx.systems);
            self.drawing_stroke = true;

            if !self.draw_fill {
                let stroke = self.calc_stroke();
                let stroke_width = ctx.systems.prefs.get::<PencilStrokeWidthPref>();
                editor.preview.stroke_preview = Some(malvina::StrokeMesh::new(ctx.device, &stroke, stroke_width));
            }
        }
    }

    fn mouse_dragged(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.prev_mouse_pos = pos;

        if !self.drawing_stroke {
            return;
        }

        let stabilized_pos = self.stabilize_point(pos, &mut ctx.systems.prefs);
        self.add_point(malvina::StrokePoint {
            pt: stabilized_pos,
            pressure: ctx.pressure,
        }, ctx.systems);

        // Update the preview
        if !self.draw_fill {
            let stroke = self.calc_stroke();
            let stroke_width = ctx.systems.prefs.get::<PencilStrokeWidthPref>();
            editor.preview.stroke_preview = Some(malvina::StrokeMesh::new(ctx.device, &stroke, stroke_width));
        } else {
            let fill = self.calc_fill();
            editor.preview.fill_preview = Some(malvina::FillMesh::new(ctx.device, &fill));
        }
    }

    fn mouse_released(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: malvina::Vec2) {
        if self.pts.is_empty() || !self.drawing_stroke {
            return;
        }

        let stabilized_pos = self.stabilize_point(pos, &mut ctx.systems.prefs);
        self.add_point(malvina::StrokePoint {
            pt: stabilized_pos,
            pressure: ctx.pressure,
        }, ctx.systems);

        if !self.draw_fill {
            let stroke = self.calc_stroke();
            Self::create_stroke(editor, ctx, stroke); 
        } else {
            let fill = self.calc_fill();
            Self::create_fill(editor, ctx, fill); 
        }

        self.drawing_stroke = false;
        self.pts.clear();
    }

    fn settings(&mut self, ui: &mut pierro::UI, systems: &mut AppSystems) {
        pierro::scroll_area(ui, |ui| {
            pierro::margin(ui, pierro::Margin::same(3.0), |ui| {
                pierro::key_value_layout(ui, |builder| {
                    self.settings_contents(builder, systems);
                });
            });
        });
    }

    fn cursor_icon(&self, _editor: &mut EditorState, _ctx: &mut ToolContext, _pos: elic::Vec2) -> pierro::CursorIcon {
        pierro::CursorIcon::Crosshair
    }

    fn render_overlay(&self, ctx: &mut ToolContext, rndr: &mut malvina::LayerRenderer, accent_color: elic::Color) {
        if let Some(prev_pt) = self.pts.last() {
            let prev_pt = prev_pt.pt;
            let stabilization_radius = ctx.systems.prefs.get::<StabilizationRadius>() as f32;

            let slack = (stabilization_radius - prev_pt.distance(self.prev_mouse_pos)).max(0.0);
            let droop = slack * 0.6 * elic::Vec2::NEG_Y;
            let mut rope = elic::BezierSegment::straight(prev_pt, self.prev_mouse_pos);
            rope.b0 += droop; 
            rope.a1 += droop;

            for i in 0..20 {
                let t0 = (i as f32) / 20.0;
                let t1 = (i as f32 + 1.0) / 20.0;
                rndr.overlay_line(rope.sample(t0), rope.sample(t1), accent_color);
            }
        } 
    }

}
