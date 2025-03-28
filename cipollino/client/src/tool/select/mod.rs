
use std::collections::HashSet;

use project::{Action, Client, Ptr, SetStrokeStroke, Stroke, StrokeData};
use crate::{EditorState, Selection};

use super::{Tool, ToolContext};

enum DragState {
    None,
    Lasso(Vec<elic::Vec2>),
    Move(elic::Vec2)
}

pub struct SelectTool {
    select_bounding_box: Option<elic::Rect>,
    select_bounding_box_version: u64,
    select_bounding_box_transform: elic::Mat4,
    drag_state: DragState,
    prev_drag_mouse_pos: elic::Vec2
}

impl Default for SelectTool {
    fn default() -> Self {
        Self {
            select_bounding_box: None,
            select_bounding_box_version: 0,
            select_bounding_box_transform: elic::Mat4::IDENTITY,
            drag_state: DragState::None,
            prev_drag_mouse_pos: elic::Vec2::ZERO
        }
    }
}

impl SelectTool {

    fn recalculate_bounding_box(&mut self, client: &Client, selection: &Selection) {
        let mut bounds = None;
        for stroke in selection.iter::<Stroke>() {
            let Some(stroke) = client.get(stroke) else { continue; };
            for segment in stroke.stroke.0.path.iter_segments() {
                let segment = segment.map(|pt| pt.pt); 
                let segment_bounds = segment.bounds(); 
                bounds = Some(bounds.map(|bounds: elic::Rect| bounds.merge(segment_bounds)).unwrap_or(segment_bounds));
            }
        }
        self.select_bounding_box = bounds;
        self.select_bounding_box_version = selection.version();
        self.select_bounding_box_transform = elic::Mat4::IDENTITY;
    }

    fn lasso_selection(&mut self, client: &Client, rendered_strokes: &HashSet<Ptr<Stroke>>, selection: &mut Selection, pts: Vec<elic::Vec2>) {
        
        let segments = pts.windows(2).map(|pts| {
            elic::Segment::new(pts[0], pts[1])
        });
        let segments_2 = segments.clone();
        let inside_lasso = |pt: elic::Vec2| {
            let mut cnt = 0;
            let line = elic::Line::horizontal(pt.y);
            for segment in segments.clone() {
                if let Some(intersection) = segment.intersect_line(line) {
                    if intersection.x > pt.x {
                        cnt += 1;
                    }
                }
            }
            cnt % 2 == 1 
        };
        let segments = segments_2;

        'stroke_loop: for stroke_ptr in rendered_strokes.iter() {
            let stroke = client.get(*stroke_ptr);
            if stroke.is_none() {
                continue;
            } 
            let stroke = stroke.unwrap();

            macro_rules! select {
                () => {
                    selection.select(*stroke_ptr); 
                    continue 'stroke_loop;
                };
            }

            let stroke_path = &stroke.stroke.0.path;

            for pt in &stroke_path.pts {
                if inside_lasso(pt.pt.pt) {
                    select!();
                } 
            }
            
            for stroke_segment in stroke_path.iter_segments() {
                let stroke_segment = stroke_segment.map(|pt| pt.pt);
                for lasso_segment in segments.clone() {
                    let ts = lasso_segment.intersect_bezier_ts(&stroke_segment);
                    if !ts.is_empty() {
                        select!();
                    }
                }
            } 
        }

    }

    fn apply_transform(&mut self, client: &Client, editor: &mut EditorState, transform: elic::Mat4) {
        let mut action = Action::new(editor.action_context("Transform strokes"));
        for stroke_ptr in editor.selection.iter::<Stroke>() {
            let Some(stroke) = client.get(stroke_ptr) else { continue; };
            let new_stroke_path = stroke.stroke.0.path.map(|pt|
                malvina::StrokePoint {
                    pt: transform.transform(pt.pt),
                    pressure: pt.pressure,
                }
            );
            action.push(SetStrokeStroke {
                ptr: stroke_ptr,
                stroke_value: StrokeData(malvina::Stroke { path: new_stroke_path }),
            });
        }
        client.queue_action(action);

        self.select_bounding_box_transform = transform * self.select_bounding_box_transform;
    }

    fn curr_transform(&self) -> elic::Mat4 {
        self.select_bounding_box_transform * match self.drag_state {
            DragState::Move(drag) => elic::Mat4::translate(drag),
            _ => elic::Mat4::IDENTITY
        }
    }

}

impl Tool for SelectTool {

    const ICON: &'static str = pierro::icons::CURSOR;
    const SHORTCUT: pierro::KeyboardShortcut = pierro::KeyboardShortcut::new(
        pierro::KeyModifiers::empty(),
        pierro::Key::V
    );

    fn mouse_drag_started(&mut self, ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.prev_drag_mouse_pos = pos;

        if let Some((x, y)) = ctx.picking_mouse_pos {
            let id = ctx.picking_buffer.read_pixel(ctx.device, ctx.queue, x, y);
            let ptr = Ptr::<Stroke>::from_key(id as u64);
            if !ptr.is_null() {
                if !ctx.editor.selection.selected(ptr) && !ctx.editor.selection.shift_down() {
                    ctx.editor.selection.clear();
                }
                ctx.editor.selection.select(ptr);
                self.drag_state = DragState::Move(elic::Vec2::ZERO);
                return;
            }
        } 
        self.drag_state = DragState::Lasso(vec![pos]);
    }

    fn mouse_dragged(&mut self, _ctx: &mut ToolContext, pos: malvina::Vec2) {
        match &mut self.drag_state {
            DragState::None => {},
            DragState::Lasso(pts) => {
                if let Some(last) = pts.last() {
                    if last.distance(pos) < 0.5 {
                        return;
                    }
                }
                pts.push(pos);
            },
            DragState::Move(drag) => {
                *drag += pos - self.prev_drag_mouse_pos;
            },
        }
        self.prev_drag_mouse_pos = pos;
    }

    fn mouse_drag_stopped(&mut self, ctx: &mut ToolContext, pos: malvina::Vec2) {
        match std::mem::replace(&mut self.drag_state, DragState::None) {
            DragState::None => {},
            DragState::Lasso(mut pts) => {
                pts.push(pos);

                if let Some(first) = pts.first() {
                    pts.push(*first);
                    self.lasso_selection(&ctx.project.client, ctx.rendered_strokes, &mut ctx.editor.selection, pts);
                }
            },
            DragState::Move(drag) => {
                self.apply_transform(&ctx.project.client, ctx.editor, elic::Mat4::translate(drag));
            },
        }
    }

    fn mouse_clicked(&mut self, ctx: &mut ToolContext, _pos: malvina::Vec2) {
        if let Some((x, y)) = ctx.picking_mouse_pos {
            let id = ctx.picking_buffer.read_pixel(ctx.device, ctx.queue, x, y);
            let ptr = Ptr::<Stroke>::from_key(id as u64);
            ctx.editor.selection.extend_select(ptr);
        }
    }

    fn tick(&mut self, ctx: &mut ToolContext) {
        if ctx.editor.selection.version() != self.select_bounding_box_version {
            self.recalculate_bounding_box(&ctx.project.client, &ctx.editor.selection);
        } 

        match &self.drag_state {
            DragState::Move(_) => {
                if ctx.editor.will_undo || ctx.editor.will_redo {
                    ctx.editor.will_undo = false;
                    self.drag_state = DragState::None;
                } else {
                    ctx.editor.preview.selection_transform = self.curr_transform();
                    ctx.editor.preview.keep_preview = true;
                }
            },
            _ => {}
        }
    }

    fn render_overlay(&self, rndr: &mut malvina::LayerRenderer, accent_color: elic::Color) {
        match &self.drag_state {
            DragState::Lasso(pts) => {
                if pts.len() >= 2 {
                    for i in 0..(pts.len() - 1) {
                        let a = pts[i];
                        let b = pts[i + 1];
                        rndr.overlay_line(a, b, accent_color);
                    }
                }
            },
            _ => {}
        }

        if let Some(bounding_box) = self.select_bounding_box {
            let transform = self.curr_transform();
            let tl = transform.transform(bounding_box.tl());
            let tr = transform.transform(bounding_box.tr());
            let bl = transform.transform(bounding_box.bl());
            let br = transform.transform(bounding_box.br());
            rndr.overlay_line(tl, tr, accent_color);
            rndr.overlay_line(tr, br, accent_color);
            rndr.overlay_line(br, bl, accent_color);
            rndr.overlay_line(bl, tl, accent_color);
        }
    }

    fn cursor_icon(&self, ctx: &mut ToolContext, _pos: elic::Vec2) -> pierro::CursorIcon {
        match self.drag_state {
            DragState::Lasso(_) => {
                return pierro::CursorIcon::Default;
            },
            DragState::Move(_) => {
                return pierro::CursorIcon::Move;
            },
            _ => {}
        }
        if let Some((x, y)) = ctx.picking_mouse_pos {
            let id = ctx.picking_buffer.read_pixel(ctx.device, ctx.queue, x, y);
            let ptr = Ptr::<Stroke>::from_key(id as u64);
            if !ptr.is_null() {
                return pierro::CursorIcon::Move;
            }
        }
        pierro::CursorIcon::default()
    }

}
