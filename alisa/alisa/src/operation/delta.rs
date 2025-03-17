
use crate::{Project, ProjectContextMut};

pub(crate) struct Delta<P: Project> {
    deltas: Vec<Box<dyn Fn(&mut ProjectContextMut<P>) + Send + Sync>>
}

impl<P: Project> Delta<P> {

    pub fn new() -> Self {
        Self {
            deltas: Vec::new(),
        }
    }

    pub fn push<F: Fn(&mut ProjectContextMut<P>) + Send + Sync + 'static>(&mut self, delta: F) {
        self.deltas.push(Box::new(delta));
    }

    pub fn undo(&self, context: &mut ProjectContextMut<P>) {
        for delta in self.deltas.iter().rev() {
            delta(context);
        }
    }

}
