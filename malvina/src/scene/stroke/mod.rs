
use std::ops::{Add, Mul};

use crate::{BezierPath, BezierPoint, HasMagnitude, Linear, Turnable};

mod meshgen;

#[derive(Clone, Copy)]
pub struct StrokePoint {
    pub pt: glam::Vec2,
    pub pressure: f32
}

impl StrokePoint {

    pub fn new(pt: glam::Vec2, pressure: f32) -> Self {
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

impl Linear for StrokePoint {}

impl HasMagnitude for StrokePoint {

    fn magnitude(&self) -> f32 {
        self.pt.length()
    }

}

impl Turnable for StrokePoint {

    fn turn_cw(&self) -> Self {
        Self {
            pt: self.pt.turn_cw(),
            pressure: self.pressure
        }
    }

}

#[derive(Clone)]
pub struct Stroke {
    pub path: BezierPath<StrokePoint>
}

impl Stroke {

    pub fn empty() -> Self {
        Self {
            path: BezierPath::empty()
        }
    }  

    pub fn point(pt: glam::Vec2, pressure: f32) -> Self {
        Self {
            path: BezierPath {
                pts: vec![
                    BezierPoint {
                        prev: StrokePoint::new(pt, pressure),
                        pt: StrokePoint::new(pt, pressure),
                        next: StrokePoint::new(pt, pressure),
                    }
                ],
            },
        }
    }

}
