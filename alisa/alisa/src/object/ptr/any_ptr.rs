
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnyPtr(pub(crate) u64);

impl AnyPtr {

    pub fn null() -> Self {
        Self(0)
    }

}

impl Default for AnyPtr {

    fn default() -> Self {
        Self::null()
    }

}

impl Debug for AnyPtr {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Ptr").field(&self.0).finish()
    }

}