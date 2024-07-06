
use crate::client::ProjectClient;

use super::{obj::ObjPtr, Project};

include!("action.gen.rs");

pub struct Action {
    pub(crate) acts: Vec<ObjAction>
}

impl Action {

    pub fn new() -> Self {
        Self {
            acts: Vec::new()
        }
    }

    pub(crate) fn add_act(&mut self, act: ObjAction) {
        self.acts.push(act); 
    }

    pub(crate) fn redo(self, project: &mut Project, client: &mut ProjectClient) -> Action {
        let mut inv_action = Action::new();
        for act in self.acts.into_iter().rev() {
            if let Some(inv_act) = act.redo(project, client) {
                inv_action.add_act(inv_act);
            }
        }
        inv_action
    }

    pub(crate) fn undo(self, project: &mut Project, client: &mut ProjectClient) -> Action {
        let mut inv_action = Action::new();
        for act in self.acts.into_iter().rev() {
            if let Some(inv_act) = act.undo(project, client) {
                inv_action.add_act(inv_act);
            }
        }
        inv_action
    }

}

pub struct ActionManager {
    undo_stack: Vec<Action>,
    redo_stack: Vec<Action>
}

impl ActionManager {

    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new()
        }
    }

    pub fn push_action(&mut self, action: Action) {
        self.undo_stack.push(action);
        self.redo_stack.clear();
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn redo(&mut self, project: &mut Project, client: &mut ProjectClient) {
        if let Some(action) = self.redo_stack.pop() {
            self.undo_stack.push(action.redo(project, client));
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }
    
    pub fn undo(&mut self, project: &mut Project, client: &mut ProjectClient) {
        if let Some(action) = self.undo_stack.pop() {
            self.redo_stack.push(action.undo(project, client));
        } 
    }

}
