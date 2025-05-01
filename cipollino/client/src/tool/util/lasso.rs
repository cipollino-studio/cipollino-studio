use std::collections::HashSet;

use project::{Client, Fill, Ptr, SceneObjPtr, Stroke};

#[derive(Default)]
pub struct LassoState {
    pts: Vec<elic::Vec2>
}

impl LassoState {

    pub fn new() -> Self {
        Self {
            pts: Vec::new(),
        }
    }
    
    pub fn from_point(pt: elic::Vec2) -> Self {
        Self {
            pts: vec![pt],
        }
    }

    pub fn add_point(&mut self, pt: elic::Vec2) {
        if let Some(prev_pt) = self.pts.last() {
            if prev_pt.distance(pt) < 0.5 {
                return;
            }
        }
        self.pts.push(pt);
    }

    pub fn clear(&mut self) {
        self.pts.clear();
    }

    pub fn find_inside(&self, client: &Client, objs: &HashSet<SceneObjPtr>) -> Vec<SceneObjPtr> {
        if self.pts.len() < 2 {
            return Vec::new();
        } 

        let segments = (0..self.pts.len()).map(|i| {
            elic::Segment::new(self.pts[i], self.pts[(i + 1) % self.pts.len()])
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

        let stroke_inside = |stroke_ptr: Ptr<Stroke>| {
            let stroke = client.get(stroke_ptr);
            if stroke.is_none() {
                return false;
            } 
            let stroke = stroke.unwrap();

            let stroke_path = &stroke.stroke.0.path;

            for pt in &stroke_path.pts {
                if inside_lasso(pt.pt.pt) {
                    return true;
                } 
            }
            
            for stroke_segment in stroke_path.iter_segments() {
                let stroke_segment = stroke_segment.map(|pt| pt.pt);
                for lasso_segment in segments.clone() {
                    let ts = lasso_segment.intersect_bezier_ts(&stroke_segment);
                    if !ts.is_empty() {
                        return true;
                    }
                }
            }

            false
        };

        let fill_inside = |fill_ptr: Ptr<Fill>| {
            let Some(fill) = client.get(fill_ptr) else { return false; };

            for path in fill.paths.0.paths.iter() {
                for path_segment in path.iter_segments() {
                    for lasso_segment in segments.clone() {
                        let ts = lasso_segment.intersect_bezier_ts(&path_segment);
                        if !ts.is_empty() {
                            return true;
                        }
                    }
                }

                for pt in path.pts.iter() {
                    if inside_lasso(pt.pt) {
                        return true;
                    }
                }
                
                let start = path.sample(0.0);
                let end = path.sample((path.pts.len() - 1) as f32);
                let start_to_end = elic::Segment::new(start, end);
                for lasso_segment in segments.clone() {
                    if lasso_segment.intersect(start_to_end).is_some() {
                        return true;
                    } 
                }
            }

            false
        };

        objs.iter().copied().filter(|obj_ptr| {
            match obj_ptr {
                SceneObjPtr::Stroke(ptr) => stroke_inside(*ptr),
                SceneObjPtr::Fill(ptr) => fill_inside(*ptr),
            } 
        }).collect()
    } 

    pub fn render_overlay(&self, rndr: &mut malvina::LayerRenderer, color: elic::Color) {
        if self.pts.len() >= 2 {
            for i in 0..(self.pts.len() - 1) {
                let a = self.pts[i];
                let b = self.pts[i + 1];
                rndr.overlay_line(a, b, color);
            }
        }
    }

}
