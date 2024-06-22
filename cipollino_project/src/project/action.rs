
use crate::client::ProjectClient;

use super::{folder::Folder, obj::ObjPtr, Project};

pub enum ObjAction {
    SetFolderName(ObjPtr<Folder>, String, String) 
}

impl ObjAction {

    pub fn redo(&self, project: &mut Project, client: &mut ProjectClient) {
        match self {
            ObjAction::SetFolderName(folder, _old_name, new_name) => client.set_folder_name_no_action(project, *folder, new_name.clone()),
        };
    }

    pub fn undo(&self, project: &mut Project, client: &mut ProjectClient) {
        match self {
            ObjAction::SetFolderName(folder, old_name, _new_name) => client.set_folder_name_no_action(project, *folder, old_name.clone()),
        };
    }

}

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

    pub(crate) fn redo(&self, project: &mut Project, client: &mut ProjectClient) {
        for act in &self.acts {
            act.redo(project, client);
        }
    }

    pub(crate) fn undo(&self, project: &mut Project, client: &mut ProjectClient) {
        for act in self.acts.iter().rev() {
            act.undo(project, client);
        }
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
            action.redo(project, client);
            self.undo_stack.push(action);
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }
    
    pub fn undo(&mut self, project: &mut Project, client: &mut ProjectClient) {
        if let Some(action) = self.undo_stack.pop() {
            action.undo(project, client);
            self.redo_stack.push(action);
        } 
    }

}
