
use std::collections::HashSet;

use project::{Client, Ptr, Stroke};

use crate::Selection;

use super::SelectTool;

impl SelectTool {

    pub(super) fn lasso_selection(&mut self, client: &Client, rendered_strokes: &HashSet<Ptr<Stroke>>, selection: &mut Selection, pts: Vec<elic::Vec2>) {
        
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

}
