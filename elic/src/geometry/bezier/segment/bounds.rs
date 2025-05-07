
use crate::{Range, Rect, Vec2};
use super::{BezierSegment, BezierSegmentExtrema};

impl BezierSegment<f32> {

    pub fn bounds(&self) -> Range {
        let mut min = self.p0.min(self.p1);
        let mut max = self.p0.max(self.p1);

        let extrema = self.extrema();
        match extrema {
            BezierSegmentExtrema::One(x) => {
                min = min.min(x); 
                max = max.max(x);
            },
            BezierSegmentExtrema::Two(x, y) => {
                min = min.min(x); 
                max = max.max(x);
                min = min.min(y); 
                max = max.max(y);
            },
            _ => {}
        }
        
        Range::new(min, max) 
    }

}

impl BezierSegment<Vec2> {

    pub fn bounds(&self) -> Rect {
        let x_bounds = self.map(|pt| pt.x).bounds();
        let y_bounds = self.map(|pt| pt.y).bounds();
        Rect::from_ranges(x_bounds, y_bounds)
    }

}
