
use crate::{vec2, Vec2};

use super::Line;

#[derive(Clone, Copy)]
pub struct Segment {
    pub a: Vec2,
    pub b: Vec2
}

impl Segment {

    pub fn new(a: Vec2, b: Vec2) -> Self {
        Self { a, b }
    }

    pub fn intersect(&self, other: Segment) -> Option<Vec2> {
        let p = self.line().intersect(other.line())?;
        if !self.potentially_contains_point(p) {
            return None;
        }
        Some(p)
    }

    pub fn line(&self) -> Line {
        Line {
            v: vec2(self.a.y - self.b.y, self.b.x - self.a.x),
            x: self.a.y * (self.b.x - self.a.x) - self.a.x * (self.b.y - self.a.y)
        }
    }

    fn potentially_contains_point(&self, p: Vec2) -> bool {
        if (self.a.x - self.b.x).abs() > (self.a.y - self.b.y).abs() {
            let min_x = self.a.x.min(self.b.x);
            let max_x = self.a.x.max(self.b.x);
            p.x >= min_x && p.x <= max_x
        } else {
            let min_y = self.a.y.min(self.b.y);
            let max_y = self.a.y.max(self.b.y);
            p.y >= min_y && p.y <= max_y
        }
    } 

}
