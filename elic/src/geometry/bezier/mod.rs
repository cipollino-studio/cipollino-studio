
use super::Linear;

mod segment;
pub use segment::*;

mod path;
pub use path::*;

#[derive(Clone, Copy, Default)]
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

    pub fn map<R: Linear, F: Fn(&T) -> R>(&self, map: F) -> BezierPoint<R> {
        BezierPoint {
            prev: map(&self.prev),
            pt: map(&self.pt),
            next: map(&self.next)
        }
    }

}
