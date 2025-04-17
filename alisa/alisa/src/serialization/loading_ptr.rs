use std::{fmt::Debug, hash::Hash};

use crate::{Object, Ptr};

use super::{DeserializationContext, Serializable, SerializationContext};

/// A reference to an object that indicates that the object refered to should be loaded from disk/the server when the referer is loaded. 
#[derive(Default)]
pub struct LoadingPtr<O: Object> {
    ptr: Ptr<O>
}

impl<O: Object> LoadingPtr<O> {

    pub fn new(ptr: Ptr<O>) -> Self {
        Self {
            ptr
        }
    }

    pub fn ptr(&self) -> Ptr<O> {
        self.ptr
    }

}

impl<O: Object> Serializable for LoadingPtr<O> {

    fn deserialize(data: &rmpv::Value, context: &mut DeserializationContext) -> Option<Self> {
        let key = data.as_u64()?; 
        context.request_load(O::TYPE_ID, key);
        Some(Self {
            ptr: Ptr::from_key(key)
        })
    }

    fn serialize(&self, context: &SerializationContext) -> rmpv::Value {
        context.request_serialize(O::TYPE_ID, self.ptr.key); 
        self.ptr.key.into()
    }

}

impl<O: Object> Clone for LoadingPtr<O> {

    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr
        }
    }

}

impl<O: Object> Copy for LoadingPtr<O> {}

impl<O: Object> PartialEq for LoadingPtr<O> {

    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }

}

impl<O: Object> Eq for LoadingPtr<O> {}

impl<O: Object> Hash for LoadingPtr<O> {

    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }

}

impl<O: Object> Debug for LoadingPtr<O> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.ptr().fmt(f)
    }

}

impl<O: Object> From<Ptr<O>> for LoadingPtr<O> {

    fn from(ptr: Ptr<O>) -> Self {
        Self::new(ptr)
    }

}
