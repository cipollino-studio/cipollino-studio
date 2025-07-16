
use std::{fmt::Debug, hash::Hash};

use crate::{ABFValue, DeserializationContext, Object, Ptr, Serializable, SerializationContext};

/// A reference to an object that indicates the pointee should be deleted when the pointer is deleted.
#[derive(Default)]
pub struct HoldingPtr<O: Object> {
    ptr: Ptr<O>
}

impl<O: Object> HoldingPtr<O> {

    pub fn new(ptr: Ptr<O>) -> Self {
        Self {
            ptr
        }
    }

    pub fn ptr(&self) -> Ptr<O> {
        self.ptr
    }

}

impl<O: Object> Serializable for HoldingPtr<O> {

    fn deserialize(data: &ABFValue, _context: &mut DeserializationContext) -> Option<Self> {
        let (obj_type, key) = data.as_obj_ptr()?; 
        if obj_type != O::TYPE_ID {
            return None;
        }
        Some(Self {
            ptr: Ptr::from_key(key)
        })
    }

    fn serialize(&self, _context: &SerializationContext) -> ABFValue {
        ABFValue::ObjPtr(O::TYPE_ID, self.ptr.key)
    }

    fn delete(&self, queue: &mut Vec<super::AnyPtr>) {
        queue.push(self.ptr.any());
    }

}

impl<O: Object> Clone for HoldingPtr<O> {

    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr
        }
    }

}

impl<O: Object> Copy for HoldingPtr<O> {}

impl<O: Object> PartialEq for HoldingPtr<O> {

    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }

}

impl<O: Object> Eq for HoldingPtr<O> {}

impl<O: Object> Hash for HoldingPtr<O> {

    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }

}

impl<O: Object> Debug for HoldingPtr<O> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.ptr().fmt(f)
    }

}

impl<O: Object> From<Ptr<O>> for HoldingPtr<O> {

    fn from(ptr: Ptr<O>) -> Self {
        Self::new(ptr)
    }

}

impl<O: Object> From<HoldingPtr<O>> for Ptr<O> {

    fn from(ptr: HoldingPtr<O>) -> Self {
        ptr.ptr() 
    }

}
