
use crate::{roots_quadratic, SmallArr};

use super::BezierSegment;

#[derive(Clone, Copy)]
pub enum BezierSegmentExtrema {
    /// There are no extrema, bezier is monotonic
    None,
    /// The bezier has slope 0 everywhere
    All,
    One(f32),
    Two(f32, f32)
}

impl BezierSegmentExtrema {

    pub fn map<F: Fn(f32) -> f32>(&self, map: F) -> BezierSegmentExtrema {
        match self {
            BezierSegmentExtrema::None => BezierSegmentExtrema::None,
            BezierSegmentExtrema::All => BezierSegmentExtrema::All,
            BezierSegmentExtrema::One(x) => BezierSegmentExtrema::One(map(*x)),
            BezierSegmentExtrema::Two(x, y) => BezierSegmentExtrema::Two(map(*x), map(*y)),
        }
    }

    pub fn into_small_arr(&self) -> SmallArr<f32, 2> {
        match self {
            BezierSegmentExtrema::None => SmallArr::empty(),
            BezierSegmentExtrema::All => SmallArr::empty(),
            BezierSegmentExtrema::One(x) => SmallArr::from_slice(&[*x]),
            BezierSegmentExtrema::Two(x, y) => SmallArr::from_slice(&[*x, *y]),
        }
    }

}

impl BezierSegment<f32> {

    pub fn extrema_ts(&self) -> BezierSegmentExtrema {
        let x = self.b0 - self.p0;
        let y = self.a1 - self.b0;
        let z = self.p1 - self.a1;
        let a = 3.0 * x - 6.0 * y + 3.0 * z;
        let b = -6.0 * x + 6.0 * y;
        let c = 3.0 * x;

        if a.abs() < 0.0001 && b.abs() < 0.0001 && c.abs() < 0.0001 {
            return BezierSegmentExtrema::All;
        }

        let mut ts = roots_quadratic(a, b, c);

        if ts.is_empty() {
            return BezierSegmentExtrema::None;
        }
        
        ts.retain(|t| 0.0 <= t && t <= 1.0);

        match ts.len() {
            2 => BezierSegmentExtrema::Two(ts[0], ts[1]),
            1 => BezierSegmentExtrema::One(ts[0]),
            0 => BezierSegmentExtrema::None,
            _ => unreachable!()
        }
    }

    pub fn extrema(&self) -> BezierSegmentExtrema {
        self.extrema_ts().map(|t| self.sample(t))
    }

}
