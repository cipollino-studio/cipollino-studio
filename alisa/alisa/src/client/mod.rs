

use std::{any::{type_name, TypeId}, cell::RefCell};

use crate::{Act, Action, Object, Operation, OperationDyn, Project, ProjectContext, ProjectContextMut, Ptr, Recorder};

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

pub struct Client<P: Project> {
    pub(crate) kind: ClientKind<P>,
    pub(crate) project: P,
    pub(crate) objects: P::Objects,
    operations_to_perform: RefCell<Vec<Box<dyn OperationDyn<Project = P>>>>,
    project_modified: bool
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

    pub fn perform<O: Operation<Project = P> + 'static>(&self, action: &mut Action<P>, operation: O) {

        // In debug mode, check that the operation being performed is registered in the project.
        // Until Rust has proper reflection, this is the best we can do :(
        #[cfg(debug_assertions)]
        {
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

        let inverse = operation.inverse(&self.context());
        self.perform_dyn(Box::new(operation));
        if let Some(inverse) = inverse {
            let act = Act {
                operation: Box::new(inverse),
            };
            action.push(act);
        }
    }

    pub(crate) fn perform_dyn(&self, operation: Box<dyn OperationDyn<Project = P>>) {
        self.operations_to_perform.borrow_mut().push(operation);
    }

    /// Update the client. Performs all the queued operations. Returns the messages that should be sent to the server.
    pub fn tick(&mut self, context: &mut P::Context) {
        let mut operations = self.operations_to_perform.borrow_mut();
        let operations = &mut *operations;
        let operations = std::mem::replace(operations, Vec::new());

        // Perform queued operations 
        for operation in operations {
            let mut recorder = Recorder::new(ProjectContextMut {
                project: &mut self.project,
                objects: &mut self.objects,
                context,
                project_modified: &mut self.project_modified,
            });
            operation.perform(&mut recorder);
            let deltas = recorder.deltas;

            if let Some(collab) = self.kind.as_collab() {
                collab.perform_operation(operation, deltas); 
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

    pub fn take_messages(&self) -> Vec<rmpv::Value> {
        match &self.kind {
            ClientKind::Local(_) => Vec::new(),
            ClientKind::Collab(collab) => collab.take_messages(),
        }
    }

    pub fn project(&self) -> &P {
        &self.project
    }

    pub fn get<O: Object<Project = P>>(&self, ptr: Ptr<O>) -> Option<&O> {
        O::list(&self.objects).get(ptr)
    }

    pub fn request_load<O: Object<Project = P>>(&self, ptr: Ptr<O>) {
        match &self.kind {
            ClientKind::Local(_) => { O::list(&self.objects).to_load.borrow_mut().insert(ptr); },
            ClientKind::Collab(collab) => {
                collab.send_message(rmpv::Value::Map(vec![
                    ("type".into(), "load".into()),
                    ("object".into(), O::NAME.into()),
                    ("key".into(), ptr.key.into()),
                ]));
            },
        }
    }

    pub(crate) fn context(&self) -> ProjectContext<P> {
        ProjectContext {
            project: &self.project,
            objects: &self.objects,
        }
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
