
use super::{HasMagnitude, Linear, Turnable};

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

pub fn sample<T: Linear>(a: &BezierPoint<T>, b: &BezierPoint<T>, t: f32) -> T {
    a.pt.clone() * ((1.0 - t) * (1.0 - t) * (1.0 - t)) +
    a.next.clone() * (3.0 * (1.0 - t) * (1.0 - t) * t) + 
    b.prev.clone() * (3.0 * (1.0 - t) * t * t) + 
    b.pt.clone() * (t * t * t) 
}

pub fn sample_derivative<T: Linear>(a: &BezierPoint<T>, b: &BezierPoint<T>, t: f32) -> T {
    a.pt.clone() * (-3.0 * (1.0 - t) * (1.0 - t)) + 
    a.next.clone() * (3.0 * (1.0 - t) * (1.0 - t) - 6.0 * (1.0 - t) * t) + 
    b.prev.clone() * (6.0 * (1.0 - t) * t - 3.0 * t * t) + 
    b.pt.clone() * (3.0 * t * t)
}

pub fn sample_tangent<T: HasMagnitude>(a: &BezierPoint<T>, b: &BezierPoint<T>, t: f32) -> T {
    let derivative = sample_derivative(a, b, t);
    let magnitude = derivative.magnitude();
    derivative * (1.0 / magnitude)
}

pub fn sample_normal<T: HasMagnitude + Turnable>(a: &BezierPoint<T>, b: &BezierPoint<T>, t: f32) -> T {
    sample_tangent(a, b, t).turn_cw()
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
        sample(a, b, t)
    }

    pub fn sample_derivative(&self, t: f32) -> T {
        let (a, b, t) = self.get_points(t);
        sample_derivative(a, b, t)
    }

}

impl<T: HasMagnitude> BezierPath<T> {

    pub fn sample_tangent(&self, t: f32) -> T {
        let (a, b, t) = self.get_points(t);
        sample_tangent(a, b, t)
    }

}

impl<T: HasMagnitude + Turnable> BezierPath<T> {

    pub fn sample_normal(&self, t: f32) -> T {
        let (a, b, t) = self.get_points(t);
        sample_normal(a, b, t)
    }

}
