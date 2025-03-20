
use crate::Rect;

use super::Painter;

impl Painter<'_> {

    pub fn push_clip_rect(&mut self, rect: Rect) {
        let rect = self.curr_transform() * rect;
        let prev_clip = self.curr_clip_rect();
        self.clip_stack.push(rect.intersect(prev_clip));
    }

    pub fn pop_clip_rect(&mut self) {
        self.clip_stack.pop();
    }

    pub fn curr_clip_rect(&self) -> Rect {
        let Some(clip_rect) = self.clip_stack.last() else { panic!("clip stack empty. painter in an invalid state.") };
        *clip_rect
    }

    pub fn with_clip_rect<F: FnOnce(&mut Self)>(&mut self, rect: Rect, contents: F) {
        self.push_clip_rect(rect);
        contents(self);
        self.pop_clip_rect();
    }

}
