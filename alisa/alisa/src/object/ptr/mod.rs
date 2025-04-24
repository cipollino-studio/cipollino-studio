
mod any_ptr;
pub use any_ptr::*;

mod loading_ptr;
pub use loading_ptr::*;

mod ptr_enum;

use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

use super::Object;

/// A reference to an object
pub struct Ptr<Obj: Object> {
    /// The unique key of the object being pointed to
    pub(crate) key: u64,
    _marker: PhantomData<Obj>
}

impl<Obj: Object> Clone for Ptr<Obj> {

    fn clone(&self) -> Self {
        Self { key: self.key.clone(), _marker: self._marker.clone() }
    }

}

impl<Obj: Object> Copy for Ptr<Obj> {}

impl<Obj: Object> Ptr<Obj> {

    pub fn from_key(key: u64) -> Self {
        Self {
            key,
            _marker: PhantomData,
        }
    }

    pub fn null() -> Self {
        Self::from_key(0)
    }

    pub fn is_null(&self) -> bool {
        *self == Self::null()
    }

    pub fn any(&self) -> AnyPtr {
        AnyPtr::new(Obj::TYPE_ID, self.key)
    }

    pub fn key(&self) -> u64 {
        self.key
    }

}

impl<Obj: Object> PartialEq for Ptr<Obj> {

    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }

}

impl<Obj: Object> Eq for Ptr<Obj> {}

impl<Obj: Object> Hash for Ptr<Obj> {

    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }

}

impl<Obj: Object> Default for Ptr<Obj> {

    fn default() -> Self {
        Self::null()
    }

}

impl<Obj: Object> Debug for Ptr<Obj> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(Obj::NAME).field(&self.key).finish()
    }
}
