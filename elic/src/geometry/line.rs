
use crate::{vec2, Vec2};

#[derive(Clone, Copy)]
pub struct Line {
    // Lines are defined by a vector V and number X such that for all points P on the line, P dot V = X.
    pub v: Vec2,
    pub x: f32
}

impl Line {

    pub fn intersect(&self, other: Line) -> Option<Vec2> {
        let v1 = self.v;
        let x1 = self.x;
        let v2 = other.v;
        let x2 = self.x;

        let k = v1.x * v2.y - v1.y * v2.x;
        if k.abs() < 0.00001 {
            return None;
        }

        let px = (x1 * v2.y - x2 * v1.y) / k;
        let py = (x1 * v2.x - x2 * v1.x) / -k;
        Some(vec2(px, py))
    }    

}
