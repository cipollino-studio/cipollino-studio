
use crate::Linear;

use super::{BezierPoint, BezierSegment};

#[derive(Clone)]
pub struct BezierPath<T: Linear> {
    pub pts: Vec<BezierPoint<T>>
}

impl<T: Linear> Default for BezierPath<T> {

    fn default() -> Self {
        Self { pts: Vec::new() }
    }

}

impl<T: Linear> BezierPath<T> {

    pub fn empty() -> Self {
        Self {
            pts: Vec::new()
        }
    }

    pub fn get_points(&self, t: f32) -> (BezierSegment<T>, f32) {
        if self.pts.is_empty() {
            panic!("bezier path empty.");
        }
        if self.pts.len() == 1 {
            return (BezierSegment::from_points(self.pts[0].clone(), self.pts[0].clone()), 0.0);
        }
        let idx = t.floor() as i32;
        let idx = (idx.max(0) as usize).min(self.pts.len() - 2);
        (BezierSegment::from_points(self.pts[idx].clone(), self.pts[idx + 1].clone()), t - (idx as f32))
    }

    pub fn sample(&self, t: f32) -> T {
        let (segment, t) = self.get_points(t);
        segment.sample(t)
    }

    pub fn sample_derivative(&self, t: f32) -> T {
        let (segment, t) = self.get_points(t);
        segment.sample_derivative(t)
    }

    pub fn iter_segments(&self) -> impl Iterator<Item = BezierSegment<T>> + '_ {
        self.pts.windows(2).map(|pts| BezierSegment::from_points(pts[0].clone(), pts[1].clone()))
    }

    pub fn map<R: Linear, F: Fn(&T) -> R>(&self, map: F) -> BezierPath<R> {
        BezierPath {
            pts: self.pts.iter().map(|pt| pt.map(&map)).collect(),
        }
    }

}
