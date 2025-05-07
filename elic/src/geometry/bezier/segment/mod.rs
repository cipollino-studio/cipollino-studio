
use crate::{Linear, Vec2};

use super::BezierPoint;

mod extrema;
pub use extrema::*;

mod bounds;

mod parallel;

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

    pub fn split(&self, t: f32) -> (BezierSegment<T>, BezierSegment<T>) {
        let m0 = T::lerp(self.p0.clone(), self.b0.clone(), t); 
        let m1 = T::lerp(self.b0.clone(), self.a1.clone(), t); 
        let m2 = T::lerp(self.a1.clone(), self.p1.clone(), t); 
        
        let q0 = T::lerp(m0.clone(), m1.clone(), t);
        let q1 = T::lerp(m1, m2.clone(), t);

        let pt = self.sample(t);

        let seg_0t = BezierSegment {
            p0: self.p0.clone(),
            b0: m0,
            a1: q0,
            p1: pt.clone(),
        };
        let seg_t1 = BezierSegment {
            p0: pt,
            b0: q1,
            a1: m2,
            p1: self.p1.clone(),
        };

        (seg_0t, seg_t1) 
    }

}

impl BezierSegment<Vec2> {

    pub fn sample_tangent(&self, t: f32) -> Vec2 {
        self.sample_derivative(t).normalize()
    }

    pub fn sample_bezier_normal(&self, t: f32) -> Vec2 {
        self.sample_tangent(t).turn_cw()
    }
    
}
