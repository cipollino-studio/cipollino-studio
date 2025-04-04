use crate::{Range, Rect, Vec2};

use super::Linear;

#[derive(Clone, Copy)]
pub struct BezierPoint<T: Linear> {
    pub prev: T,
    pub pt: T,
    pub next: T
}

impl<T: Linear> BezierPoint<T> {

    pub fn new(prev: T, pt: T, next: T) -> Self {
        Self {
            prev,
            pt,
            next,
        }
    }

    pub fn map<R: Linear, F: Fn(&T) -> R>(&self, map: F) -> BezierPoint<R> {
        BezierPoint {
            prev: map(&self.prev),
            pt: map(&self.pt),
            next: map(&self.next)
        }
    }

}

#[derive(Clone, Copy)]
pub struct BezierSegment<T: Linear> {
    pub p0: T,
    pub b0: T,
    pub a1: T,
    pub p1: T 
}

impl<T: Linear> BezierSegment<T> {

    pub fn from_points(a: BezierPoint<T>, b: BezierPoint<T>) -> Self {
        Self {
            p0: a.pt,
            b0: a.next,
            a1: b.prev,
            p1: b.pt
        }
    }

    pub fn sample(&self, t: f32) -> T {
        let p0 = self.p0.scale((1.0 - t) * (1.0 - t) * (1.0 - t));
        let p1 = self.b0.scale(3.0 * (1.0 - t) * (1.0 - t) * t); 
        let p2 = self.a1.scale(3.0 * (1.0 - t) * t * t);
        let p3 = self.p1.scale(t * t * t);
        p0.add(p1).add(p2).add(p3)
    }

    pub fn sample_derivative(&self, t: f32) -> T {
        let p0 = self.p0.scale(-3.0 * (1.0 - t) * (1.0 - t));
        let p1 = self.b0.scale(3.0 * (1.0 - t) * (1.0 - t) - 6.0 * (1.0 - t) * t);
        let p2 = self.a1.scale(6.0 * (1.0 - t) * t - 3.0 * t * t);
        let p3 = self.p1.scale(3.0 * t * t);
        p0.add(p1).add(p2).add(p3)
    }

    pub fn map<R: Linear, F: Fn(&T) -> R>(&self, map: F) -> BezierSegment<R> {
        BezierSegment {
            p0: map(&self.p0),
            b0: map(&self.b0),
            a1: map(&self.a1),
            p1: map(&self.p1),
        }
    }

}

impl BezierSegment<f32> {

    pub fn bounds(&self) -> Range {
        let mut min = self.p0.min(self.p1);
        let mut max = self.p0.max(self.p1);
        
        let x = self.b0 - self.p0;
        let y = self.a1 - self.b0;
        let z = self.p1 - self.a1;
        let a = 3.0 * x - 6.0 * y + 3.0 * z;
        let b = -6.0 * x + 6.0 * y;
        let c = 3.0 * x;
        let det = b * b - 4.0 * a * c;

        if det > 0.0 {
            let t1 = (-b + det.sqrt()) / (2.0 * a);
            if 0.0 <= t1 && t1 <= 1.0 {
                let val = self.sample(t1);
                min = min.min(val);
                max = max.max(val);
            } 
            let t2 = (-b - det.sqrt()) / (2.0 * a);
            if 0.0 <= t2 && t2 <= 1.0 {
                let val = self.sample(t2);
                min = min.min(val);
                max = max.max(val);
            } 
        }

        Range::new(min, max) 
    }

}

impl BezierSegment<Vec2> {

    pub fn sample_tangent(&self, t: f32) -> Vec2 {
        self.sample_derivative(t).normalize()
    }

    pub fn sample_bezier_normal(&self, t: f32) -> Vec2 {
        self.sample_tangent(t).turn_cw()
    }
    
    pub fn bounds(&self) -> Rect {
        let x_bounds = self.map(|pt| pt.x).bounds();
        let y_bounds = self.map(|pt| pt.y).bounds();
        Rect::from_ranges(x_bounds, y_bounds)
    }

}

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
