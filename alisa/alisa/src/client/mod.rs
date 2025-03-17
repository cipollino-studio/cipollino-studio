

use std::{any::{type_name, TypeId}, cell::RefCell, ops::Deref};

use crate::{Act, Action, Delta, Object, Operation, OperationSource, Project, ProjectContext, ProjectContextMut, Ptr, Recorder};

mod local;
use local::*;

mod collab;
use collab::*;

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

    fn next_key(&self) -> Option<u64> {
        match self {
            ClientKind::Local(local) => Some(local.next_key()),
            ClientKind::Collab(collab) => collab.next_key(),
        }
    }

    fn has_keys(&self) -> bool {
        match self {
            ClientKind::Local(_local) => true,
            ClientKind::Collab(collab) => collab.has_keys(),
        }
    }

}

enum OperationToPerform<P: Project> {
    Operation(Act<P>),
    Action(Action<P>),
    Undo,
    Redo
}

pub struct Client<P: Project> {
    pub(crate) kind: ClientKind<P>,
    pub(crate) project: P,
    pub(crate) objects: P::Objects,
    operations_to_perform: RefCell<Vec<OperationToPerform<P>>>,
    project_modified: bool,
    undo_stack: Vec<Action<P>>,
    redo_stack: Vec<Action<P>>
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

    pub fn next_ptr<O: Object>(&self) -> Option<Ptr<O>> {
        self.kind.next_key().map(Ptr::from_key)
    }

    pub fn has_keys(&self) -> bool {
        self.kind.has_keys()
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

        self.operations_to_perform.borrow_mut().push(OperationToPerform::Operation(Act::new(operation))); 
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

    pub fn undo(&self) {
        self.operations_to_perform.borrow_mut().push(OperationToPerform::Undo);
    }

    pub fn redo(&self) {
        self.operations_to_perform.borrow_mut().push(OperationToPerform::Redo);
    }

    fn perform_act(&mut self, act: Act<P>, context: &mut P::Context) {
        let mut delta = Delta::new();
        let mut recorder = Recorder::new(ProjectContextMut {
            project: &mut self.project,
            objects: &mut self.objects,
            context,
            project_modified: &mut self.project_modified,
        }, OperationSource::Local, Some(&mut delta));
        let success = act.operation.perform(&mut recorder);

        if success {
            if let Some(collab) = self.kind.as_collab() {
                collab.perform_operation(act.operation, delta); 
            }
        } else {
            let mut context = ProjectContextMut {
                project: &mut self.project,
                objects: &mut self.objects,
                context,
                project_modified: &mut self.project_modified,
            };

            // If the operation failed, undo the mess it made
            delta.undo(&mut context);
        }
    }

    fn perform_action(&mut self, action: Action<P>, context: &mut P::Context) -> Action<P> {
        let mut inverse_acts = Vec::new();
        for act in action.acts {
            if let Some(inverse) = act.operation.inverse(&self.context()) {
                inverse_acts.push(Act {
                    operation: inverse,
                });
            }
            self.perform_act(act, context);
        }
        inverse_acts.reverse();
        Action {
            acts: inverse_acts,
            context: action.context
        }
    }

    /// Update the client. Performs all the queued operations. Returns the messages that should be sent to the server.
    pub fn tick(&mut self, context: &mut P::Context) {
        let mut operations_ref = self.operations_to_perform.borrow_mut();
        let operations = &mut *operations_ref;
        let operations = std::mem::replace(operations, Vec::new());
        drop(operations_ref);

        // Perform queued operations 
        for operation in operations {
            match operation {
                OperationToPerform::Operation(act) => self.perform_act(act, context),
                OperationToPerform::Action(action) => {
                    let inv_action = self.perform_action(action, context);
                    if !inv_action.is_empty() {
                        self.undo_stack.push(inv_action);
                    }
                    self.redo_stack.clear();
                },
                OperationToPerform::Undo => {
                    if let Some(undo_action) = self.undo_stack.pop() {
                        let redo_action = self.perform_action(undo_action, context);
                        if !redo_action.is_empty() {
                            self.redo_stack.push(redo_action);
                        }
                    }
                },
                OperationToPerform::Redo => {
                    if let Some(redo_action) = self.redo_stack.pop() {
                        let undo_action = self.perform_action(redo_action, context);
                        if !undo_action.is_empty() {
                            self.undo_stack.push(undo_action);
                        }
                    }
                },
            }
        }

        if let Some(collab) = self.kind.as_collab() {
            collab.request_keys(); 
        }

        if let Some(local) = self.kind.as_local() {
            local.save_changes(&mut self.project, &mut self.objects, &mut self.project_modified);
            local.load_objects(&mut self.objects);
        }
    }

    pub fn has_messages(&self) -> bool {
        match &self.kind {
            ClientKind::Local(_) => false,
            ClientKind::Collab(collab) => collab.has_messages(),
        }
    }

    pub fn take_messages(&self) -> Vec<rmpv::Value> {
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

    pub fn get<O: Object<Project = P>>(&self, ptr: Ptr<O>) -> Option<&O> {
        O::list(&self.objects).get(ptr)
    }

    pub fn request_load<O: Object<Project = P>>(&self, ptr: Ptr<O>) {
        if O::list(&self.objects).tried_loading.borrow().contains(&ptr) {
            return;
        }
        O::list(&self.objects).to_load.borrow_mut().insert(ptr);
        O::list(&self.objects).tried_loading.borrow_mut().insert(ptr);
        match &self.kind {
            ClientKind::Local(_) => {  },
            ClientKind::Collab(collab) => {
                collab.send_message(rmpv::Value::Map(vec![
                    ("type".into(), "load".into()),
                    ("object".into(), O::NAME.into()),
                    ("key".into(), ptr.key.into()),
                ]));
            },
        }
    }

    pub fn has_load_failed<O: Object<Project = P>>(&self, ptr: Ptr<O>) -> bool {
        let tried_loading = O::list(&self.objects).tried_loading.borrow().contains(&ptr);
        let to_load = O::list(&self.objects).to_load.borrow().contains(&ptr);
        tried_loading && !to_load
    }

    pub fn undo_stack(&self) -> &Vec<Action<P>> {
        &self.undo_stack
    }

    pub fn redo_stack(&self) -> &Vec<Action<P>> {
        &self.redo_stack
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

    for i in 0..P::OBJECTS.len() {
        for j in (i + 1)..P::OBJECTS.len() {
            let a = &P::OBJECTS[i];
            let b = &P::OBJECTS[j];
            if a.name == b.name {
                panic!("duplicate object name '{}' in {}::OBJECTS. operations {} and {} have identical names.", a.name, type_name::<P>(), (a.type_name)(), (b.type_name)());
            }
        }
    }
}
