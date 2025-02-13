
mod project_context;
pub use project_context::*;

use crate::{Serializable, ObjectKind, OperationKind};

pub trait Project: Sized + Serializable<Self> + 'static {

    type Context;
    /// A struct containing an `ObjList` for every kind of object in the project
    type Objects: Default;

    fn empty() -> Self;
    fn create_default(&mut self);

    const OBJECTS: &'static [ObjectKind<Self>];
    const OPERATIONS: &'static [OperationKind<Self>];

    fn verter_config() -> verter::Config {
        verter::Config {
            magic_bytes: b"ALISA___",
            page_size: 64,
        }
    }

}
