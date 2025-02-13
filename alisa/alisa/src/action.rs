
use std::cell::RefCell;

use crate::{Client, OperationDyn, Project};

pub(crate) struct Act<P: Project> {
    pub(crate) operation: Box<dyn OperationDyn<Project = P>>
}

pub struct Action<P: Project> {
    acts: Vec<Act<P>>
}

impl<P: Project> Action<P> {

    pub fn new() -> Self {
        Self {
            acts: Vec::new()
        }
    }

    pub(crate) fn push(&mut self, act: Act<P>) {
        self.acts.push(act);
    }

    fn perform(mut self, client: &Client<P>) -> Self {
        let mut inverse_acts = Vec::new();
        self.acts.reverse();
        for act in self.acts {
            if let Some(inverse) = act.operation.inverse(&client.context()) {
                inverse_acts.push(Act { operation: inverse });
            }
            client.perform_dyn(act.operation);
        }
        inverse_acts.reverse();
        Self {
            acts: inverse_acts
        }
    }

}

pub struct UndoRedoManager<P: Project> {
    undo_stack: RefCell<Vec<Action<P>>>,
    redo_stack: RefCell<Vec<Action<P>>>
}

impl<P: Project> UndoRedoManager<P> {

    pub fn new() -> Self {
        Self {
            undo_stack: RefCell::new(Vec::new()),
            redo_stack: RefCell::new(Vec::new())
        }
    }

    pub fn add(&self, action: Action<P>) {
        self.undo_stack.borrow_mut().push(action);
        self.redo_stack.borrow_mut().clear();
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.borrow().is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.borrow().is_empty()
    }

    pub fn undo(&mut self, client: &Client<P>) {
        let Some(action) = self.undo_stack.borrow_mut().pop() else { return; };
        let redo_action = action.perform(client);
        self.redo_stack.borrow_mut().push(redo_action);
    }

    pub fn redo(&mut self, client: &Client<P>) {
        let Some(action) = self.redo_stack.borrow_mut().pop() else { return; };
        let undo_action = action.perform(client);
        self.undo_stack.borrow_mut().push(undo_action);
    }

}
