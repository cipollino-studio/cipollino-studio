
use std::{cell::RefCell, collections::HashSet};

use crate::{File, Project};

mod binary;
pub use binary::*;

mod serialization_impls;

mod loading_ptr;
pub use loading_ptr::*;

enum DeserializationContextKind<'a, P: Project> {
    Local {
        file: &'a mut File,
        objects: &'a mut P::Objects,
    },
    Collab {
        objects: &'a mut P::Objects,
    },
    Data
}

pub struct DeserializationContext<'a, P: Project> {
    kind: DeserializationContextKind<'a, P>,
    /// The keys of the objects already loaded
    loaded: HashSet<u64>,
}

impl<'a, P: Project> DeserializationContext<'a, P> {

    pub(crate) fn local(objects: &'a mut P::Objects, file: &'a mut File) -> Self {
        Self {
            kind: DeserializationContextKind::Local {
                file,
                objects
            },
            loaded: HashSet::new(),
        }
    }

    pub(crate) fn collab(objects: &'a mut P::Objects) -> Self {
        Self {
            kind: DeserializationContextKind::Collab {
                objects,
            },
            loaded: HashSet::new()
        }
    }

    pub(crate) fn data() -> Self {
        Self {
            kind: DeserializationContextKind::Data,
            loaded: HashSet::new(),
        }
    }

}

enum SerializationContextKind<'a, P: Project> {
    Shallow,
    Deep {
        objects: &'a P::Objects,
    },
}

pub struct SerializationContext<'a, P: Project> {
    kind: SerializationContextKind<'a, P>,
    /// The keys of the objects already stored
    stored: RefCell<HashSet<u64>>,
}

impl<'a, P: Project> SerializationContext<'a, P> {

    pub(crate) fn shallow() -> Self {
        Self {
            kind: SerializationContextKind::Shallow,
            stored: RefCell::new(HashSet::new()),
        }
    }

    pub(crate) fn deep(objects: &'a P::Objects) -> Self {
        Self {
            kind: SerializationContextKind::Deep {
                objects
            },
            stored: RefCell::new(HashSet::new()),
        }
    }

    pub(crate) fn with_stored(self, key: u64) -> Self {
        self.stored.borrow_mut().insert(key);
        self
    }

}

pub trait Serializable<P: Project>: Sized {

    fn serialize(&self, context: &SerializationContext<P>) -> rmpv::Value; 
    fn deserialize(data: &rmpv::Value, context: &mut DeserializationContext<P>) -> Option<Self>;

    fn shallow_serialize(&self) -> rmpv::Value {
        self.serialize(&SerializationContext::shallow())
    }

    fn data_deserialize(data: &rmpv::Value) -> Option<Self> {
        Self::deserialize(data, &mut DeserializationContext::data())
    }

}
