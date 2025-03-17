
use std::cell::RefCell;

use keychain::KeyChain;

use crate::{rmpv_get, Delta, DeserializationContext, OperationDyn, OperationSource, Project, ProjectContextMut, Recorder, UnconfirmedOperation};

use super::{Client, ClientKind};

#[cfg(debug_assertions)]
use super::verify_project_type;

mod keychain;

pub(crate) struct Collab<P: Project> {
    keychain: RefCell<KeyChain<2>>,
    key_request_sent: bool,
    unconfirmed_operations: Vec<UnconfirmedOperation<P>>,
    to_send: RefCell<Vec<rmpv::Value>>
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
            self.send_message(rmpv::Value::Map(vec![
                ("type".into(), "key_request".into())
            ]));
            self.key_request_sent = true;
        }
    }

    pub(crate) fn perform_operation(&mut self, operation: Box<dyn OperationDyn<Project = P>>, delta: Delta<P>) {
        self.send_message(rmpv::Value::Map(vec![
            ("type".into(), "operation".into()),
            ("operation".into(), operation.name().into()),
            ("data".into(), operation.serialize())
        ]));
        self.unconfirmed_operations.push(UnconfirmedOperation {
            operation,
            delta
        });
    }
    
    pub(crate) fn send_message(&self, message: rmpv::Value) {
        self.to_send.borrow_mut().push(message);
    }

    pub(crate) fn has_messages(&self) -> bool {
        !self.to_send.borrow().is_empty()
    } 

    pub(crate) fn take_messages(&self) -> Vec<rmpv::Value> {
        std::mem::replace(&mut *self.to_send.borrow_mut(), Vec::new())
    }

}

impl<P: Project> Client<P> {

    pub fn collab(welcome_data: &rmpv::Value) -> Option<Self> {

        #[cfg(debug_assertions)]
        verify_project_type::<P>();

        welcome_data.as_map()?;
        let project_data = rmpv_get(welcome_data, "project")?;
        let mut objects = P::Objects::default();
        let project = P::deserialize(project_data, &mut DeserializationContext::collab(&mut objects))?;
        Some(Self {
            kind: ClientKind::Collab(Collab::new()),
            project,
            objects,
            operations_to_perform: RefCell::new(Vec::new()),
            project_modified: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new()
        })
    }

    pub(crate) fn handle_operation_message(&mut self, operation_name: &str, data: &rmpv::Value, context: &mut P::Context) -> bool {
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
        let success = (operation_kind.perform)(operation, &mut recorder);

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
                if unconfirmed_operation.operation.perform(&mut recorder) {
                    collab.unconfirmed_operations.push(UnconfirmedOperation { operation: unconfirmed_operation.operation, delta });
                } else {
                    let mut project_context = ProjectContextMut {
                        project: &mut self.project,
                        objects: &mut self.objects,
                        context,
                        project_modified: &mut self.project_modified,
                    };
                    delta.undo(&mut project_context);
                }
            }
        }

        success
    }

    pub fn receive_message(&mut self, msg: rmpv::Value, context: &mut P::Context) -> Option<()> {

        if !self.is_collab() {
            return None;
        }

        let msg = msg.as_map()?;
        let mut msg_type = "";
        let mut operation_name = "";
        let mut data = None;
        let mut first = 0;
        let mut last = 0;
        let mut load_key = 0;
        let mut object = "";

        for (key, val) in msg {
            let key = key.as_str()?;
            match key {
                "type" => {
                    msg_type = val.as_str()?;
                },
                "operation" => {
                    operation_name = val.as_str()?;
                },
                "data" => {
                    data = Some(val);
                },
                "first" => {
                    first = val.as_u64()?;
                },
                "last" => {
                    last = val.as_u64()?;
                },
                "key" => {
                    load_key = val.as_u64()?;
                },
                "object" => {
                    object = val.as_str()?;
                },
                _ => {}
            }
        }

        match msg_type {
            "confirm" => {
                if let Some(collab) = self.kind.as_collab() {
                    collab.unconfirmed_operations.remove(0);
                }
            },
            "operation" => {
                if let Some(data) = data {
                    self.handle_operation_message(operation_name, data, context);
                }
            },
            "key_grant" => {
                if first != 0 && last != 0 {
                    if let Some(collab) = self.kind.as_collab() {
                        collab.accept_keys(first, last);
                        collab.key_request_sent = false;
                    }
                }
            },
            "load" => {
                for object_kind in P::OBJECTS {
                    if object_kind.name == object {
                        (object_kind.load_object_from_message)(&mut self.objects, load_key, data?);
                        break;
                    }
                }
            },
            "load_failed" => {
                for object_kind in P::OBJECTS {
                    if object_kind.name == object {
                        (object_kind.load_failed)(&mut self.objects, load_key); 
                    }
                } 
            }
            _ => {}
        }

        Some(())
    }
   
}
