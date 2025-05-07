
use crate::{roots_quadratic, SmallArr, Vec2};
use super::BezierSegment;

impl BezierSegment<Vec2> {
    
    /// Finds the ts such that the tangent to the bezier is parallel to the segment p0-p1. 
    pub fn parallel_ts(&self) -> SmallArr<f32, 2> {
        let dir = self.p1 - self.p0;
        self.parallel_to_ts(dir)
    }

    /// Finds the ts such that the tangent to the bezier is parallel to the direction vector dir 
    pub fn parallel_to_ts(&self, dir: Vec2) -> SmallArr<f32, 2> {
        let dir = dir.turn_cw();

        let x = (self.b0 - self.p0).dot(dir);
        let y = (self.a1 - self.b0).dot(dir);
        let z = (self.p1 - self.a1).dot(dir);
        let a = 3.0 * x - 6.0 * y + 3.0 * z;
        let b = -6.0 * x + 6.0 * y;
        let c = 3.0 * x;

        let mut ts = roots_quadratic(a, b, c);
        ts.retain(|t| 0.0 <= t && t <= 1.0);

        ts
    }

}
