
use crate::{Operation, OperationDyn, Project};

pub(crate) struct Act<P: Project> {
    pub(crate) operation: Box<dyn OperationDyn<Project = P>>
}

impl<P: Project> Act<P> {

    pub fn new<O: Operation<Project = P>>(operation: O) -> Self {
        Act {
            operation: Box::new(operation)
        } 
    }

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

    pub fn push<O: Operation<Project = P>>(&mut self, operation: O) {
        self.acts.push(Act {
            operation: Box::new(operation),
        });
    }

    pub fn single<O: Operation<Project = P>>(context: P::ActionContext, operation: O) -> Self {
        let mut action = Action::new(context);
        action.push(operation);
        action
    }

    pub fn iter_operations(&self) -> impl Iterator<Item = &Box<dyn OperationDyn<Project = P>>> {
        self.acts.iter().map(|act| &act.operation)
    }

    pub fn is_empty(&self) -> bool {
        self.acts.is_empty()
    }

}
