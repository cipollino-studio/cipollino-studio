
use std::cell::RefCell;

mod abf;
pub use abf::*;

mod serialization_impls;

mod loading_ptr;
pub use loading_ptr::*;

pub struct DeserializationContext {
    pub(crate) load_requests: Vec<(u16, u64)>,
}

impl DeserializationContext {

    pub(crate) fn new() -> Self {
        Self {
            load_requests: Vec::new(),
        }
    } 

    pub(crate) fn request_load(&mut self, obj_type: u16, key: u64) {
        self.load_requests.push((obj_type, key));
    }

}

pub struct SerializationContext {
    serialization_requests: RefCell<Vec<(u16, u64)>>,
}

impl SerializationContext {

    pub(crate) fn new() -> Self {
        Self {
            serialization_requests: RefCell::new(Vec::new())
        }
    } 

    /// Request that another object also be serialized and sent to the client
    pub(crate) fn request_serialize(&self, obj_type: u16, key: u64) {
        self.serialization_requests.borrow_mut().push((obj_type, key));
    }

    pub(crate) fn take_serialization_requests(self) -> Vec<(u16, u64)> {
        self.serialization_requests.take()
    }

}

pub trait Serializable: Sized {

    fn serialize(&self, context: &SerializationContext) -> ABFValue; 
    fn deserialize(data: &ABFValue, context: &mut DeserializationContext) -> Option<Self>;

    fn shallow_serialize(&self) -> ABFValue {
        self.serialize(&SerializationContext::new())
    }

    fn data_deserialize(data: &ABFValue) -> Option<Self> {
        Self::deserialize(data, &mut DeserializationContext::new())
    }

}
