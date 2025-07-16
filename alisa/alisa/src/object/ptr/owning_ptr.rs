
use std::{fmt::Debug, hash::Hash};

use crate::{ABFValue, DeserializationContext, Object, Ptr, Serializable, SerializationContext};

/// A reference to an object that indicates that the object refered to should be loaded from disk/the server when the referer is loaded,
/// and that the pointee should be deleted when the pointer is deleted.
#[derive(Default)]
pub struct OwningPtr<O: Object> {
    ptr: Ptr<O>
}

impl<O: Object> OwningPtr<O> {

    pub fn new(ptr: Ptr<O>) -> Self {
        Self {
            ptr
        }
    }

    pub fn ptr(&self) -> Ptr<O> {
        self.ptr
    }

}

impl<O: Object> Serializable for OwningPtr<O> {

    fn deserialize(data: &ABFValue, context: &mut DeserializationContext) -> Option<Self> {
        let (obj_type, key) = data.as_obj_ptr()?; 
        if obj_type != O::TYPE_ID {
            return None;
        }
        context.request_load(O::TYPE_ID, key);
        Some(Self {
            ptr: Ptr::from_key(key)
        })
    }

    fn serialize(&self, context: &SerializationContext) -> ABFValue {
        context.request_serialize(O::TYPE_ID, self.ptr.key); 
        ABFValue::ObjPtr(O::TYPE_ID, self.ptr.key)
    }

    fn delete(&self, queue: &mut Vec<super::AnyPtr>) {
        queue.push(self.ptr.any());
    }

}

impl<O: Object> Clone for OwningPtr<O> {

    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr
        }
    }

}

impl<O: Object> Copy for OwningPtr<O> {}

impl<O: Object> PartialEq for OwningPtr<O> {

    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }

}

impl<O: Object> Eq for OwningPtr<O> {}

impl<O: Object> Hash for OwningPtr<O> {

    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }

}

impl<O: Object> Debug for OwningPtr<O> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.ptr().fmt(f)
    }

}

impl<O: Object> From<Ptr<O>> for OwningPtr<O> {

    fn from(ptr: Ptr<O>) -> Self {
        Self::new(ptr)
    }

}

impl<O: Object> From<OwningPtr<O>> for Ptr<O> {

    fn from(ptr: OwningPtr<O>) -> Self {
        ptr.ptr() 
    }

}