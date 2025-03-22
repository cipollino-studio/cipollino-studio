use crate::Vec2;

use super::Linear;


#[derive(Clone)]
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

}

pub fn sample_bezier<T: Linear>(a: &BezierPoint<T>, b: &BezierPoint<T>, t: f32) -> T {
    let p0 = a.pt.scale((1.0 - t) * (1.0 - t) * (1.0 - t));
    let p1 = a.next.scale(3.0 * (1.0 - t) * (1.0 - t) * t); 
    let p2 = b.prev.scale(3.0 * (1.0 - t) * t * t);
    let p3 = b.pt.scale(t * t * t);
    p0.add(p1).add(p2).add(p3)
}

pub fn sample_bezier_derivative<T: Linear>(a: &BezierPoint<T>, b: &BezierPoint<T>, t: f32) -> T {
    let p0 = a.pt.scale(-3.0 * (1.0 - t) * (1.0 - t));
    let p1 = a.next.scale(3.0 * (1.0 - t) * (1.0 - t) - 6.0 * (1.0 - t) * t);
    let p2 = b.prev.scale(6.0 * (1.0 - t) * t - 3.0 * t * t);
    let p3 = b.pt.scale(3.0 * t * t);
    p0.add(p1).add(p2).add(p3)
}

pub fn sample_bezier_tangent(a: &BezierPoint<Vec2>, b: &BezierPoint<Vec2>, t: f32) -> Vec2 {
    sample_bezier_derivative(a, b, t).normalize()
}

pub fn sample_bezier_normal(a: &BezierPoint<Vec2>, b: &BezierPoint<Vec2>, t: f32) -> Vec2 {
    sample_bezier_tangent(a, b, t).turn_cw()
}

#[derive(Clone)]
pub struct BezierPath<T: Linear> {
    pub pts: Vec<BezierPoint<T>>
}

impl<T: Linear> BezierPath<T> {

    pub fn empty() -> Self {
        Self {
            pts: Vec::new()
        }
    }

    pub fn get_points(&self, t: f32) -> (&BezierPoint<T>, &BezierPoint<T>, f32) {
        if self.pts.is_empty() {
            panic!("bezier path empty.");
        }
        if self.pts.len() == 1 {
            return (&self.pts[0], &self.pts[0], 0.0);
        }
        let idx = t.floor() as i32;
        let idx = (idx.max(0) as usize).min(self.pts.len() - 2);
        (&self.pts[idx], &self.pts[idx + 1], t - (idx as f32))
    }

    pub fn sample(&self, t: f32) -> T {
        let (a, b, t) = self.get_points(t);
        sample_bezier(a, b, t)
    }

    pub fn sample_derivative(&self, t: f32) -> T {
        let (a, b, t) = self.get_points(t);
        sample_bezier_derivative(a, b, t)
    }

}
