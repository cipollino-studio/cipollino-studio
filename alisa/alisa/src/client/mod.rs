

use std::{any::{type_name, TypeId}, cell::RefCell, ops::Deref};

use crate::{Act, Action, Delta, Message, ObjRef, Object, Operation, OperationDyn, OperationSource, Project, ProjectContext, ProjectContextMut, Ptr, Recorder};

mod local;
use local::*;

mod collab;
pub(crate) use collab::*;

pub(crate) enum ClientKind<P: Project> {
    Local(Local<P>),
    Collab(Collab<P>)
}

impl<P: Project> ClientKind<P> {

    pub(crate) fn as_local(&mut self) -> Option<&mut Local<P>> {
        match self {
            ClientKind::Local(local) => Some(local),
            ClientKind::Collab(..) => None,
        }
    }

    pub(crate) fn as_collab(&mut self) -> Option<&mut Collab<P>> {
        match self {
            ClientKind::Local(..) => None,
            ClientKind::Collab(collab) => Some(collab),
        }
    }

    fn next_key(&self) -> u64 {
        match self {
            ClientKind::Local(local) => local.next_key(),
            ClientKind::Collab(collab) => collab.next_key(),
        }
    }

}

enum OperationToPerform<P: Project> {
    Operation(Box<dyn OperationDyn<Project = P>>),
    Action(Action<P>),
    Undo(Action<P>),
    Redo(Action<P>)
}

pub struct Client<P: Project> {
    pub(crate) kind: ClientKind<P>,
    pub(crate) project: P,
    pub(crate) objects: P::Objects,
    operations_to_perform: RefCell<Vec<OperationToPerform<P>>>,
    project_modified: bool,
    undo_stack: RefCell<Vec<Action<P>>>,
    redo_stack: RefCell<Vec<Action<P>>>
}

impl<P: Project> Client<P> {

    pub fn is_local(&self) -> bool {
        match &self.kind {
            ClientKind::Local(..) => true,
            ClientKind::Collab(..) => false,
        }
    }

    pub fn is_collab(&self) -> bool {
        match &self.kind {
            ClientKind::Local(..) => false,
            ClientKind::Collab(..) => true,
        }
    }

    pub fn next_key(&self) -> u64 {
        self.kind.next_key()
    }

    pub fn next_ptr<O: Object>(&self) -> Ptr<O> {
        Ptr::from_key(self.next_key())
    }

    /// In debug mode, check that the operation being performed is registered in the project.
    /// Until Rust has proper reflection, this is the best we can do :(
    #[cfg(debug_assertions)]
    pub(crate) fn verify_operation_type<O: Operation<Project = P>>() {
        let mut found = false;
        for operation_kind in P::OPERATIONS {
            if (operation_kind.type_id)() == TypeId::of::<O>() {
                found = true;
            }
        }
        if !found {
            panic!("operation '{}' not registered in {}::OPERATIONS.", O::NAME, type_name::<P>());
        }
    }

    pub fn queue_operation<O: Operation<Project = P> + 'static>(&self, operation: O) {
        #[cfg(debug_assertions)]
        Self::verify_operation_type::<O>();

        self.operations_to_perform.borrow_mut().push(OperationToPerform::Operation(Box::new(operation))); 
    }

    pub fn queue_action(&self, action: Action<P>) {
        if action.is_empty() {
            return;
        }

        #[cfg(debug_assertions)]
        for act in &action.acts {
            act.operation.verify_operation_type();
        }

        self.operations_to_perform.borrow_mut().push(OperationToPerform::Action(action)); 
    }

    pub fn undo(&self) -> Option<P::ActionContext> {
        let undo_action = self.undo_stack.borrow_mut().pop()?;
        let context = undo_action.context.clone();
        self.operations_to_perform.borrow_mut().push(OperationToPerform::Undo(undo_action));
        Some(context)
    }

    pub fn redo(&self) -> Option<P::ActionContext> {
        let redo_action = self.redo_stack.borrow_mut().pop()?;
        let context = redo_action.context.clone(); 
        self.operations_to_perform.borrow_mut().push(OperationToPerform::Redo(redo_action));
        Some(context)
    }

    fn perform_act(&mut self, operation: Box<dyn OperationDyn<Project = P>>) {
        let mut delta = Delta::new();
        let mut recorder = Recorder::new(ProjectContextMut {
            project: &mut self.project,
            objects: &mut self.objects,
            project_modified: &mut self.project_modified,
        }, OperationSource::Local, Some(&mut delta));
        let success = operation.perform(&mut recorder) && *recorder.success.borrow();

        if success {
            if let Some(collab) = self.kind.as_collab() {
                collab.perform_operation(operation, delta); 
            }
        } else {
            let mut context = ProjectContextMut {
                project: &mut self.project,
                objects: &mut self.objects,
                project_modified: &mut self.project_modified,
            };

            // If the operation failed, undo the mess it made
            delta.undo(&mut context);
        }
    }

    fn perform_action(&mut self, action: Action<P>) -> Action<P> {
        let mut inverse_acts = Vec::new();
        for act in action.acts {
            if let Some(inverse) = act.operation.inverse(&self.context()) {
                inverse_acts.push(Act {
                    operation: inverse,
                });
            }
            self.perform_act(act.operation);
        }
        inverse_acts.reverse();
        Action {
            acts: inverse_acts,
            context: action.context
        }
    }

    /// Update the client. Performs all the queued operations. Returns the messages that should be sent to the server.
    pub fn tick(&mut self) {

        let mut operations_ref = self.operations_to_perform.borrow_mut();
        let operations = &mut *operations_ref;
        let operations = std::mem::replace(operations, Vec::new());
        drop(operations_ref);

        // Perform queued operations 
        for operation in operations {
            match operation {
                OperationToPerform::Operation(act) => self.perform_act(act),
                OperationToPerform::Action(action) => {
                    let inv_action = self.perform_action(action);
                    if !inv_action.is_empty() {
                        self.undo_stack.borrow_mut().push(inv_action);
                    }
                    self.redo_stack.borrow_mut().clear();
                },
                OperationToPerform::Undo(undo_action) => {
                    let redo_action = self.perform_action(undo_action);
                    if !redo_action.is_empty() {
                        self.redo_stack.borrow_mut().push(redo_action);
                    }
                },
                OperationToPerform::Redo(redo_action) => {
                    let undo_action = self.perform_action(redo_action);
                    if !undo_action.is_empty() {
                        self.undo_stack.borrow_mut().push(undo_action);
                    }
                },
            }
        }

        if let Some(collab) = self.kind.as_collab() {
            collab.load_objects(&mut self.objects);
        }

        if let Some(local) = self.kind.as_local() {
            local.save_changes(&mut self.project, &mut self.objects, &mut self.project_modified);
            local.load_objects(&mut self.objects);
        }

        // Clear modifications for the next tick 
        for object_kind in P::OBJECTS {
            (object_kind.clear_modifications)(&mut self.objects);
        }

    }

    pub fn has_messages(&self) -> bool {
        match &self.kind {
            ClientKind::Local(_) => false,
            ClientKind::Collab(collab) => collab.has_messages(),
        }
    }

    pub fn take_messages(&self) -> Vec<Message> {
        match &self.kind {
            ClientKind::Local(_) => Vec::new(),
            ClientKind::Collab(collab) => collab.take_messages(),
        }
    }

    pub fn project(&self) -> &P {
        &self.project
    }

    pub fn context(&self) -> ProjectContext<P> {
        ProjectContext {
            project: &self.project,
            objects: &self.objects
        }
    }

    pub fn get<O: Object<Project = P>, T: Into<Ptr<O>>>(&self, ptr: T) -> Option<&O> {
        O::list(&self.objects).get(ptr.into())
    }

    pub fn get_ref<O: Object<Project = P>, T: Into<Ptr<O>>>(&self, ptr: T) -> ObjRef<O> {
        O::list(&self.objects).get_ref(ptr.into())
    }

    pub fn request_load<O: Object<Project = P>, T: Into<Ptr<O>>>(&self, ptr: T) {
        O::list(&self.objects).to_load.borrow_mut().insert(ptr.into());
    }

    pub fn undo_stack(&self) -> &RefCell<Vec<Action<P>>> {
        &self.undo_stack
    }

    pub fn redo_stack(&self) -> &RefCell<Vec<Action<P>>> {
        &self.redo_stack
    }

    pub fn modified<O: Object<Project = P>>(&self) -> impl Iterator<Item = Ptr<O>> + '_ {
        O::list(&self.objects).user_modified.iter().copied()
    }

    pub fn clear_modified<O: Object<Project = P>>(&mut self) {
        O::list_mut(&mut self.objects).user_modified.clear();
    }

    pub fn clear_all_modified(&mut self) {
        for obj_kind in P::OBJECTS {
            (obj_kind.clear_user_modified)(&mut self.objects); 
        }
    }

}

impl<P: Project> Deref for Client<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.project()
    }
}

#[cfg(debug_assertions)]
fn verify_project_type<P: Project>() {
    for i in 0..P::OPERATIONS.len() {
        for j in (i + 1)..P::OPERATIONS.len() {
            let a = &P::OPERATIONS[i];
            let b = &P::OPERATIONS[j];
            if a.name == b.name {
                panic!("duplicate operation name '{}' in {}::OPERATIONS. operations {} and {} have identical names.", a.name, type_name::<P>(), (a.type_name)(), (b.type_name)());
            }
        }
    }
}
