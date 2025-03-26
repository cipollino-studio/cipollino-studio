
use std::ops::{Add, Mul};

mod meshgen;

#[derive(Clone, Copy)]
pub struct StrokePoint {
    pub pt: elic::Vec2,
    pub pressure: f32
}

impl StrokePoint {

    pub fn new(pt: elic::Vec2, pressure: f32) -> Self {
        Self {
            pt,
            pressure
        }
    }

}

impl Add<StrokePoint> for StrokePoint {

    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            pt: self.pt + rhs.pt,
            pressure: self.pressure + rhs.pressure
        }
    }

}

impl Mul<f32> for StrokePoint {

    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            pt: self.pt * rhs,
            pressure: self.pressure * rhs,
        }
    }

}

impl elic::Linear for StrokePoint {

    fn add(&self, other: Self) -> Self {
        Self {
            pt: self.pt + other.pt,
            pressure: self.pressure + other.pressure,
        }
    }

    fn scale(&self, scl: f32) -> Self {
        Self {
            pt: self.pt * scl,
            pressure: self.pressure * scl,
        }
    }

}

#[derive(Clone)]
pub struct Stroke {
    pub path: elic::BezierPath<StrokePoint>,
}

impl Stroke {

    pub fn empty() -> Self {
        Self {
            path: elic::BezierPath::empty(),
        }
    }  

    pub fn point(pt: elic::Vec2, pressure: f32) -> Self {
        Self {
            path: elic::BezierPath {
                pts: vec![
                    elic::BezierPoint {
                        prev: StrokePoint::new(pt, pressure),
                        pt: StrokePoint::new(pt, pressure),
                        next: StrokePoint::new(pt, pressure),
                    }
                ],
            },
        }
    }

}
