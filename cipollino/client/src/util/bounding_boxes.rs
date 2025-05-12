
use project::{Fill, Stroke};

pub fn stroke(stroke: &Stroke) -> Option<elic::Rect> {
    let mut bounds = None;
    for segment in stroke.stroke.0.path.iter_segments() {
        let segment = segment.map(|pt| pt.pt); 
        let segment_bounds = segment.bounds(); 
        bounds = Some(bounds.map(|bounds: elic::Rect| bounds.merge(segment_bounds)).unwrap_or(segment_bounds));
    }
    for pt in &stroke.stroke.0.path.pts {
        let pt_rect = elic::Rect::min_max(pt.pt.pt, pt.pt.pt);
        bounds = Some(bounds.map(|bounds: elic::Rect| bounds.merge(pt_rect)).unwrap_or(pt_rect));
    }
    bounds
}

pub fn fill(fill: &Fill) -> Option<elic::Rect> {
    let mut bounds = None;
    for path in &fill.paths.0.paths {
        for segment in path.iter_segments() {
            let segment_bounds = segment.bounds();
            bounds = Some(bounds.map(|bounds: elic::Rect| bounds.merge(segment_bounds)).unwrap_or(segment_bounds));
        }
    }
    bounds
}
