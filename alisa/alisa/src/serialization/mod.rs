
use std::cell::RefCell;

mod abf;
pub use abf::*;

mod serialization_impls;

pub struct DeserializationContext {
    pub(crate) load_requests: Vec<(u16, u64)>,
}

impl DeserializationContext {

    pub(crate) fn new() -> Self {
        Self {
            load_requests: Vec::new(),
        }
    } 

    /// Request that another object also be loaded
    pub fn request_load(&mut self, obj_type: u16, key: u64) {
        self.load_requests.push((obj_type, key));
    }

}

pub struct SerializationContext {
    serialization_requests: RefCell<Vec<(u16, u64)>>,
}

impl SerializationContext {

    pub(crate) fn new() -> Self {
        Self {
            serialization_requests: RefCell::new(Vec::new()),
        }
    }

    /// Request that another object also be serialized and sent to the client
    pub fn request_serialize(&self, obj_type: u16, key: u64) {
        self.serialization_requests.borrow_mut().push((obj_type, key));
    }

    pub(crate) fn take_serialization_requests(self) -> Vec<(u16, u64)> {
        self.serialization_requests.take()
    }

}

pub trait Serializable: Sized {

    fn serialize(&self, context: &SerializationContext) -> ABFValue; 
    fn deserialize(data: &ABFValue, context: &mut DeserializationContext) -> Option<Self>;

}

pub fn serialize<T: Serializable>(obj: &T) -> ABFValue {
    obj.serialize(&SerializationContext::new())
}

pub fn deserialize<T: Serializable>(data: &ABFValue) -> Option<T> {
    T::deserialize(data, &mut DeserializationContext::new())
}