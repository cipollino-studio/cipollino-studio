
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
    pub(crate) acts: Vec<Act<P>>
}

impl<P: Project> Action<P> {

    pub fn new() -> Self {
        Self {
            acts: Vec::new()
        }
    }

    pub fn push<O: Operation<Project = P>>(&mut self, operation: O) {
        self.acts.push(Act {
            operation: Box::new(operation),
        });
    }

    pub fn single<O: Operation<Project = P>>(operation: O) -> Self {
        let mut action = Action::new();
        action.push(operation);
        action
    }

}
