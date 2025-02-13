use crate::TSTransform;

use super::Painter;

impl Painter<'_> {

    pub fn push_transform(&mut self, transform: TSTransform) {
        self.transform_stack.push(transform);
    }

    pub fn pop_transform(&mut self) {
        self.transform_stack.pop();
    }

    pub fn curr_transform(&self) -> TSTransform {
        let Some(transform) = self.transform_stack.last() else { panic!("transform stack empty. painter in an invalid state.") };
        *transform
    }

}