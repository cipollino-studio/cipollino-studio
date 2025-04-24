
use std::cell::RefCell;

use keychain::KeyChain;

use crate::{ABFValue, Delta, DeserializationContext, Message, OperationDyn, OperationSource, Project, ProjectContextMut, Recorder, UnconfirmedOperation, WelcomeMessage};

use super::{Client, ClientKind};

#[cfg(debug_assertions)]
use super::verify_project_type;

mod keychain;

pub(crate) struct Collab<P: Project> {
    keychain: RefCell<KeyChain<2>>,
    key_request_sent: bool,
    unconfirmed_operations: Vec<UnconfirmedOperation<P>>,
    to_send: RefCell<Vec<Message>>
}

impl<P: Project> Collab<P> {

    pub(crate) fn new() -> Self {
        Self {
            keychain: RefCell::new(KeyChain::new()),
            key_request_sent: false,
            unconfirmed_operations: Vec::new(),
            to_send: RefCell::new(Vec::new())
        }
    }

    pub(crate) fn next_key(&self) -> Option<u64> {
        self.keychain.borrow_mut().next_key()
    }

    pub(crate) fn has_keys(&self) -> bool {
        self.keychain.borrow().has_keys()
    }

    pub(crate) fn accept_keys(&self, first: u64, last: u64) {
        self.keychain.borrow_mut().accept_keys(first, last);
    }

    pub(crate) fn request_keys(&mut self) {
        let keychain = self.keychain.borrow_mut();
        if keychain.wants_keys() && !self.key_request_sent {
            self.send_message(Message::KeyRequest);
            self.key_request_sent = true;
        }
    }

    pub(crate) fn load_objects(&mut self, objects: &mut P::Objects) {
        for object_kind in P::OBJECTS {
            (object_kind.collab_load_objects)(objects, self);    
        }
    }

    pub(crate) fn perform_operation(&mut self, operation: Box<dyn OperationDyn<Project = P>>, delta: Delta<P>) {
        self.send_message(Message::Operation {
            operation: operation.name().to_owned(),
            data: operation.serialize()
        });

        self.unconfirmed_operations.push(UnconfirmedOperation {
            operation,
            delta
        });
    }
    
    pub(crate) fn send_message(&self, message: Message) {
        self.to_send.borrow_mut().push(message);
    }

    pub(crate) fn has_messages(&self) -> bool {
        !self.to_send.borrow().is_empty()
    } 

    pub(crate) fn take_messages(&self) -> Vec<Message> {
        std::mem::replace(&mut *self.to_send.borrow_mut(), Vec::new())
    }

}

impl<P: Project> Client<P> {

    pub fn collab(welcome_data: &WelcomeMessage) -> Option<Self> {
        #[cfg(debug_assertions)]
        verify_project_type::<P>();

        let mut objects = P::Objects::default();
        let project = P::deserialize(&welcome_data.project, &mut DeserializationContext::new())?;

        for object_data in &welcome_data.objects {
            let obj_type = object_data.ptr.obj_type();
            let object_kind = &P::OBJECTS[obj_type as usize];
            let key = object_data.ptr.key();
            (object_kind.load_object_from_message)(&mut objects, key, &object_data.obj);
        }

        Some(Self {
            kind: ClientKind::Collab(Collab::new()),
            project,
            objects,
            operations_to_perform: RefCell::new(Vec::new()),
            project_modified: false,
            undo_stack: RefCell::new(Vec::new()),
            redo_stack: RefCell::new(Vec::new())
        })
    }

    pub(crate) fn handle_operation_message(&mut self, operation_name: &str, data: &ABFValue, context: &mut P::Context) -> bool {
        // Find the type of operation being performed
        let Some(operation_kind) = P::OPERATIONS.iter().find(|kind| kind.name == operation_name) else {
            return false;
        };
        // Deserialize the operation from the message
        let Some(operation) = (operation_kind.deserialize)(data) else {
            return false;
        };

        let mut project_context = ProjectContextMut {
            project: &mut self.project,
            objects: &mut self.objects,
            context,
            project_modified: &mut self.project_modified,
        };

        // Undo all the stuff we've done client side
        if let Some(collab) = self.kind.as_collab() {
            for unconfirmed_operation in collab.unconfirmed_operations.iter().rev() {
                unconfirmed_operation.delta.undo(&mut project_context); 
            }
        }

        // Apply the newly-received operation
        let mut recorder = Recorder::new(project_context, OperationSource::Server, None);
        let success = (operation_kind.perform)(operation, &mut recorder) && *recorder.success.borrow();

        // Reapply the operations we've done on top of the inserted operation
        if let Some(collab) = self.kind.as_collab() {
            let unconfirmed_operations = std::mem::replace(&mut collab.unconfirmed_operations, Vec::new());
            
            for unconfirmed_operation in unconfirmed_operations {
                let mut delta = Delta::new();
                let project_context = ProjectContextMut {
                    project: &mut self.project,
                    objects: &mut self.objects,
                    context,
                    project_modified: &mut self.project_modified,
                };
                let mut recorder = Recorder::new(project_context, OperationSource::Local, Some(&mut delta));
                if !unconfirmed_operation.operation.perform(&mut recorder) && *recorder.success.borrow() {
                    let mut project_context = ProjectContextMut {
                        project: &mut self.project,
                        objects: &mut self.objects,
                        context,
                        project_modified: &mut self.project_modified,
                    };
                    delta.undo(&mut project_context);
                }
                collab.unconfirmed_operations.push(UnconfirmedOperation { operation: unconfirmed_operation.operation, delta });
            }
        }

        success
    }

    pub fn receive_message(&mut self, msg: &Message, context: &mut P::Context) {
        if !self.is_collab() {
            return;
        }

        match msg {
            Message::ConfirmOperation => {
                if let Some(collab) = self.kind.as_collab() {
                    // The check is necessary because an unconfirmed operation might fail after it is reapplied,
                    // So it might never get re-added to the unconfirmed operation queue
                    if !collab.unconfirmed_operations.is_empty() {
                        collab.unconfirmed_operations.remove(0);
                    }
                }
            },
            Message::Operation { operation, data } => {
                self.handle_operation_message(&operation, data, context);
            },
            Message::KeyGrant { first, last } => {
                if *first != 0 && *last != 0 {
                    if let Some(collab) = self.kind.as_collab() {
                        collab.accept_keys(*first, *last);
                        collab.key_request_sent = false;
                    }
                }
            },
            Message::Load { ptr, obj } => {
                let obj_type = ptr.obj_type();
                let key = ptr.key();
                let object_kind = &P::OBJECTS[obj_type as usize];
                (object_kind.load_object_from_message)(&mut self.objects, key, obj);
            },
            Message::LoadFailed { ptr } => {
                let obj_type = ptr.obj_type();
                let key = ptr.key();
                let object_kind = &P::OBJECTS[obj_type as usize];
                (object_kind.load_failed)(&mut self.objects, key); 
            },
            _ => {}
        }

    }
   
}
