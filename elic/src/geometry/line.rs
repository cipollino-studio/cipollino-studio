
use crate::{vec2, SmallArr, Vec2};

use super::BezierSegment;

#[derive(Clone, Copy)]
pub struct Line {
    // Lines are defined by a vector V and number X such that for all points P on the line, P dot V = X.
    pub v: Vec2,
    pub x: f32
}

impl Line {
    
    pub fn new(a: Vec2, b: Vec2) -> Self {
        Self {
            v: vec2(a.y - b.y, b.x - a.x),
            x: a.y * (b.x - a.x) - a.x * (b.y - a.y)
        }
    }

    pub fn horizontal(y: f32) -> Line {
        Line {
            v: Vec2::Y,
            x: y,
        }
    }

    pub fn vertical(x: f32) -> Line {
        Line {
            v: Vec2::X,
            x 
        }
    }

    pub fn intersect(&self, other: Line) -> Option<Vec2> {
        let v1 = self.v;
        let x1 = self.x;
        let v2 = other.v;
        let x2 = other.x;

        let k = v1.x * v2.y - v1.y * v2.x;
        if k.abs() < 0.00001 {
            return None;
        }

        let px = (x1 * v2.y - x2 * v1.y) / k;
        let py = (x1 * v2.x - x2 * v1.x) / -k;
        Some(vec2(px, py))
    }

    pub fn intersect_bezier_ts(&self, segment: &BezierSegment<Vec2>) -> SmallArr<f32, 3> {
        let c0 = segment.p0.dot(self.v); 
        let c1 = segment.b0.dot(self.v);
        let c2 = segment.a1.dot(self.v);
        let c3 = segment.p1.dot(self.v);

        let poly_a = -c0 + 3.0 * c1 - 3.0 * c2 + c3;
        let poly_b = 3.0 * c0 - 6.0 * c1 + 3.0 * c2;
        let poly_c = -3.0 * c0 + 3.0 * c1;
        let poly_d = c0 - self.x;

        let mut ts = match roots::find_roots_cubic(poly_a, poly_b, poly_c, poly_d) {
            roots::Roots::No(_) => SmallArr::empty(),
            roots::Roots::One([x]) => SmallArr::from_slice(&[x]),
            roots::Roots::Two([x, y]) => SmallArr::from_slice(&[x, y]),
            roots::Roots::Three([x, y, z]) => SmallArr::from_slice(&[x, y, z]),
            roots::Roots::Four(_) => panic!("should be impossible"),
        };

        ts.retain(|t| t >= 0.0 && t <= 1.0);

        ts
    }

}
