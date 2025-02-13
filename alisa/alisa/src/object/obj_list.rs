use std::{any::{type_name, TypeId}, cell::RefCell, collections::{HashMap, HashSet}};

use crate::Project;

use super::{Ptr, Object};


pub struct ObjList<Obj: Object> {
    objs: HashMap<Ptr<Obj>, Obj>,
    pub(crate) modified: HashSet<Ptr<Obj>>,
    pub(crate) to_delete: HashSet<Ptr<Obj>>,
    pub(crate) to_load: RefCell<HashSet<Ptr<Obj>>>,
}

impl<Obj: Object> ObjList<Obj> {

    pub fn insert(&mut self, ptr: Ptr<Obj>, obj: Obj) {
        if self.objs.contains_key(&ptr) {
            return;
        }
        self.objs.insert(ptr, obj);
        self.modified.insert(ptr);
    }

    pub fn delete(&mut self, ptr: Ptr<Obj>) -> Option<Obj> {
        if self.get(ptr).is_none() {
            return None;
        }
        self.to_delete.insert(ptr);
        self.objs.remove(&ptr)
    }

    pub fn get(&self, ptr: Ptr<Obj>) -> Option<&Obj> {
        self.objs.get(&ptr) 
    }

    pub fn get_mut(&mut self, ptr: Ptr<Obj>) -> Option<&mut Obj> {
        self.modified.insert(ptr);
        self.objs.get_mut(&ptr) 
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