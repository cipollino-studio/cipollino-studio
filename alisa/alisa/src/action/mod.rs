
use crate::Project;

mod invertible_operation;
pub use invertible_operation::*;

pub(crate) struct Act<P: Project> {
    pub(crate) operation: Box<dyn InvertibleOperationDyn<Project = P>>
}

pub struct Action<P: Project> {
    pub(crate) acts: Vec<Act<P>>,
    pub context: P::ActionContext
}

impl<P: Project> Action<P> {

    pub fn new(context: P::ActionContext) -> Self {
        Self {
            acts: Vec::new(),
            context
        }
    }

    pub fn push<O: InvertibleOperation<Project = P>>(&mut self, operation: O) {
        self.acts.push(Act {
            operation: Box::new(operation),
        });
    }

    pub fn single<O: InvertibleOperation<Project = P>>(context: P::ActionContext, operation: O) -> Self {
        let mut action = Action::new(context);
        action.push(operation);
        action
    }

    pub fn iter_operations(&self) -> impl Iterator<Item = &Box<dyn InvertibleOperationDyn<Project = P>>> {
        self.acts.iter().map(|act| &act.operation)
    }

    pub fn is_empty(&self) -> bool {
        self.acts.is_empty()
    }

}
