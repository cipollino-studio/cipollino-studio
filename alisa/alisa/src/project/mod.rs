
mod project_context;
pub use project_context::*;

use crate::{Client, ObjectKind, OperationKind, Serializable};

pub trait Project: Sized + Serializable + 'static + Clone + Sync + Send {

    /// A struct containing an `ObjList` for every kind of object in the project
    type Objects: Default;
    /// Some data associated with each action.
    type ActionContext: Clone;

    fn empty() -> Self;
    fn create_default(_client: &Client<Self>) {

    }

    const OBJECTS: &'static [ObjectKind<Self>];
    const OPERATIONS: &'static [OperationKind<Self>];

    fn verter_config() -> verter::Config {
        verter::Config {
            magic_bytes: b"ALISA___",
            page_size: 64,
        }
    }

}
