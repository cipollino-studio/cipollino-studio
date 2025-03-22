
use std::collections::HashSet;

use project::{Client, Ptr, Stroke};
use crate::Selection;

use super::{Tool, ToolContext};

#[derive(Default)]
pub struct SelectTool {
    lasso_pts: Vec<elic::Vec2>
}

impl SelectTool {

    fn lasso_selection(&self, client: &Client, rendered_strokes: &HashSet<Ptr<Stroke>>, selection: &mut Selection) {
        
        let segments = self.lasso_pts.windows(2).map(|pts| {
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

}

impl Tool for SelectTool {

    const ICON: &'static str = pierro::icons::CURSOR;
    const SHORTCUT: pierro::KeyboardShortcut = pierro::KeyboardShortcut::new(
        pierro::KeyModifiers::empty(),
        pierro::Key::V
    );

    fn mouse_drag_started(&mut self, _ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.lasso_pts.clear();
        self.lasso_pts.push(pos); 
    }

    fn mouse_dragged(&mut self, _ctx: &mut ToolContext, pos: malvina::Vec2) {
        if let Some(last) = self.lasso_pts.last() {
            if last.distance(pos) < 0.5 {
                return;
            }
        }
        self.lasso_pts.push(pos); 
    }

    fn mouse_drag_stopped(&mut self, ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.lasso_pts.push(pos);

        if let Some(first) = self.lasso_pts.first() {
            self.lasso_pts.push(*first);
            self.lasso_selection(&ctx.project.client, ctx.rendered_strokes, &mut ctx.editor.selection);
        }
        
        self.lasso_pts.clear();
    }

    fn mouse_clicked(&mut self, ctx: &mut ToolContext, _pos: malvina::Vec2) {
        if let Some((x, y)) = ctx.picking_mouse_pos {
            let id = ctx.picking_buffer.read_pixel(ctx.device, ctx.queue, x, y);
            let ptr = Ptr::<Stroke>::from_key(id as u64);
            ctx.editor.selection.extend_select(ptr);
        } 
    }

    fn render_overlay(&self, rndr: &mut malvina::LayerRenderer, accent_color: elic::Color) {
        if self.lasso_pts.len() < 2 {
            return;
        }
        for i in 0..(self.lasso_pts.len() - 1) {
            let a = self.lasso_pts[i];
            let b = self.lasso_pts[i + 1];
            rndr.overlay_line(a, b, accent_color);
        }
    }

}
