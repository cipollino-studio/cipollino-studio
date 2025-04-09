use std::{any::{type_name, TypeId}, cell::RefCell, collections::{HashMap, HashSet}};

use crate::Project;

use super::{Ptr, Object};

enum ObjState<Obj: Object> {
    Loading,
    Loaded(Obj),
    Deleted
}

impl<Obj: Object> ObjState<Obj> {

    pub fn as_ref(&self) -> Option<&Obj> {
        match self {
            ObjState::Loading => None,
            ObjState::Loaded(obj) => Some(obj),
            ObjState::Deleted => None,
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut Obj> {
        match self {
            ObjState::Loading => None,
            ObjState::Loaded(obj) => Some(obj),
            ObjState::Deleted => None,
        }
    }

}

impl<Obj: Object> From<ObjState<Obj>> for Option<Obj> {

    fn from(state: ObjState<Obj>) -> Self {
        match state {
            ObjState::Loading => None,
            ObjState::Loaded(obj) => Some(obj),
            ObjState::Deleted => None,
        }
    }

}

pub enum ObjRef<'a, Obj: Object> {
    None,
    Loading,
    Loaded(&'a Obj),
    Deleted
}

impl<'a, Obj: Object> ObjRef<'a, Obj> {

    pub fn is_none(&self) -> bool {
        match self {
            ObjRef::None => true,
            _ => false
        }
    }

    pub fn is_loading(&self) -> bool {
        match self {
            ObjRef::Loading => true,
            _ => false
        }
    }

    pub fn is_loaded(&self) -> bool {
        match self {
            ObjRef::Loaded(_) => true,
            _ => false
        }
    }

    pub fn is_deleted(&self) -> bool {
        match self {
            ObjRef::Deleted => true,
            _ => false
        }
    }

}

pub struct ObjList<Obj: Object> {
    objs: HashMap<Ptr<Obj>, ObjState<Obj>>,
    pub(crate) modified: HashSet<Ptr<Obj>>,
    pub(crate) to_delete: HashSet<Ptr<Obj>>,
    pub(crate) to_load: RefCell<HashSet<Ptr<Obj>>>,
}

impl<Obj: Object> ObjList<Obj> {

    pub fn insert(&mut self, ptr: Ptr<Obj>, obj: Obj) {
        if ptr.is_null() {
            return;
        }
        match self.objs.get(&ptr) {
            Some(ObjState::Loading) | Some(ObjState::Loaded(_)) => {
                return;
            },
            _ => {}
        }
        self.objs.insert(ptr, ObjState::Loaded(obj));
        self.modified.insert(ptr);
    }

    pub(crate) fn insert_loaded(&mut self, ptr: Ptr<Obj>, obj: Obj) {
        if ptr.is_null() {
            return;
        }
        match self.objs.get(&ptr) {
            Some(ObjState::Loaded(_)) => {
                return;
            },
            _ => {}
        }
        self.objs.insert(ptr, ObjState::Loaded(obj));
    }

    pub(crate) fn mark_loading(&mut self, ptr: Ptr<Obj>) {
        if !self.objs.contains_key(&ptr) {
            self.objs.insert(ptr, ObjState::Loading);
        }
    }

    pub fn delete(&mut self, ptr: Ptr<Obj>) -> Option<Obj> {
        if self.get(ptr).is_none() {
            return None;
        }
        self.to_delete.insert(ptr);
        self.objs.insert(ptr, ObjState::Deleted)?.into()
    }

    pub fn get_ref(&self, ptr: Ptr<Obj>) -> ObjRef<Obj> {
        match self.objs.get(&ptr) {
            Some(ObjState::Loading) => ObjRef::Loading,
            Some(ObjState::Loaded(obj)) => ObjRef::Loaded(obj),
            Some(ObjState::Deleted) => ObjRef::Deleted,
            None => ObjRef::None,
        }
    }

    pub fn get(&self, ptr: Ptr<Obj>) -> Option<&Obj> {
        self.objs.get(&ptr)?.as_ref()
    }

    pub fn get_mut(&mut self, ptr: Ptr<Obj>) -> Option<&mut Obj> {
        self.modified.insert(ptr);
        self.objs.get_mut(&ptr)?.as_mut()
    }

}

impl<O: Object> Default for ObjList<O> {

    fn default() -> Self {

        #[cfg(debug_assertions)]
        {
            if <O::Project as Project>::OBJECTS.iter().find(|object_kind| (object_kind.type_id)() == TypeId::of::<O>()).is_none() {
                panic!("object '{}' not registered in {}::OBJECTS.", O::NAME, type_name::<O::Project>());
            }
        }

        Self {
            objs: HashMap::new(),
            modified: HashSet::new(),
            to_delete: HashSet::new(),
            to_load: RefCell::new(HashSet::new()),
        }
    }

}