
use std::{fmt::Debug, u16};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnyPtr {
    obj_type: u16,
    key: u64,
}

impl AnyPtr {

    pub(crate) fn new(obj_type: u16, key: u64) -> Self {
        Self {
            obj_type,
            key,
        }
    }

    pub fn null() -> Self {
        Self {
            obj_type: u16::MAX,
            key: 0
        }
    }

    pub fn obj_type(&self) -> u16 {
        self.obj_type
    }

    pub fn key(&self) -> u64 {
        self.key
    }

}

impl Default for AnyPtr {

    fn default() -> Self {
        Self::null()
    }

}

impl Debug for AnyPtr {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Ptr").field(&self.key).finish()
    }

}
