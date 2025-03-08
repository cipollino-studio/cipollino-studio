
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

pub fn arc_length<T: HasMagnitude>(a: &BezierPoint<T>, b: &BezierPoint<T>, from: f32, to: f32, step_size: f32) -> f32 {
    let mut length = 0.0;
    let mut t = from;
    let mut p = sample(a, b, t);
    while t < to {
        let next = (t + step_size).min(to);
        let next_p = sample(a, b, next); 
        length += p.distance(next_p.clone());
        t = next;
        p = next_p;
        t += step_size;
    }
    length
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

    pub fn arc_length(&self, from: f32, to: f32, step_size: f32) -> f32 {
        let from = from.max(0.0).min(self.pts.len() as f32 - 1.001);
        let to = to.max(0.0).min(self.pts.len() as f32 - 1.001);
        if from.floor() == to.floor() {
            let (_, _, from_t) = self.get_points(from);
            let (a, b, to_t) = self.get_points(to);
            return arc_length(a, b, from_t, to_t, step_size)
        }

        let (from_a, from_b, from_t) = self.get_points(from);
        let from_len = arc_length(from_a, from_b, from_t, 1.0, step_size);
        let (to_a, to_b, to_t) = self.get_points(from);
        let to_len = arc_length(to_a, to_b, 0.0, to_t, step_size);

        let mut len = from_len + to_len;
        for i in (from.floor() as u32 + 1)..(to.floor() as u32) {
            let a = &self.pts[i as usize];
            let b = &self.pts[i as usize + 1];
            len += arc_length(a, b, 0.0, 1.0, step_size);
        }

        len 
    }

}

impl<T: HasMagnitude + Turnable> BezierPath<T> {

    pub fn sample_normal(&self, t: f32) -> T {
        let (a, b, t) = self.get_points(t);
        sample_normal(a, b, t)
    }

}
