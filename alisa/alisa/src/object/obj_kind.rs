
use std::{any::{type_name, TypeId}, collections::HashSet};

use crate::{DeserializationContext, File, LoadingPtr, Project, Serializable, SerializationContext};

use super::{Object, Ptr};


pub struct ObjectKind<P: Project> {
    pub(crate) name: &'static str,
    pub(crate) save_modifications: fn(&mut File, objects: &mut P::Objects),
    pub(crate) load_objects: fn(&mut File, &mut P::Objects),
    pub(crate) load_object: fn(&mut File, &mut P::Objects, u64),
    pub(crate) load_object_from_message: fn(&mut P::Objects, u64, &rmpv::Value),
    pub(crate) load_failed: fn(&mut P::Objects, u64),
    pub(crate) serialize_object: fn(&mut P::Objects, u64) -> Option<rmpv::Value>,

    #[cfg(debug_assertions)]
    pub(crate) type_id: fn() -> TypeId,
    #[cfg(debug_assertions)]
    pub(crate) type_name: fn() -> &'static str
}

fn load_object<O: Object>(file: &mut File, objects: &mut <O::Project as Project>::Objects, key: u64) {
    // Fun trick: instead of implementing loading logic, just deserialize a LoadingPtr pointing to the object we want :)
    let loading_ptr = LoadingPtr::<O>::new(Ptr::from_key(key));
    let loading_ptr_data = loading_ptr.serialize(&SerializationContext::shallow());
    LoadingPtr::<O>::deserialize(&loading_ptr_data, &mut DeserializationContext::local(objects, file));
}

impl<P: Project> ObjectKind<P> {

    pub const fn from<O: Object<Project = P>>() -> Self {
        Self {
            name: O::NAME,
            save_modifications: |file, objects| {
                for modified in std::mem::replace(&mut O::list_mut(objects).modified, HashSet::new()) {
                    if let Some(object) = O::list(objects).get(modified) {
                        let object_data = object.serialize(&SerializationContext::shallow());
                        if let Some(ptr) = file.get_ptr(modified.key) {
                            file.write(ptr, &object_data);
                        }
                    }
                }
                for deleted in std::mem::replace(&mut O::list_mut(objects).to_delete, HashSet::new()) {
                    file.delete(deleted.key);
                }
            },
            load_objects: |file, objects| {
                let to_load = std::mem::replace(&mut *O::list_mut(objects).to_load.borrow_mut(), HashSet::new());
                for ptr in to_load {
                    load_object::<O>(file, objects, ptr.key);
                }
            },
            load_object: |file, objects, key| {
                load_object::<O>(file, objects, key);
            },
            load_object_from_message: |objects, key, data| {
                if let Some(obj) = O::deserialize(data, &mut DeserializationContext::collab(objects)) {
                    O::list_mut(objects).insert(Ptr::from_key(key), obj);
                }
            },
            load_failed: |objects, key| {
                O::list_mut(objects).to_load.borrow_mut().remove(&Ptr::from_key(key)); 
            },
            serialize_object: |objects, key| {
                O::list(objects).get(Ptr::from_key(key)).map(|data| data.serialize(&SerializationContext::deep(objects).with_stored(key)))
            },
            #[cfg(debug_assertions)]
            type_id: || TypeId::of::<O>(),
            #[cfg(debug_assertions)]
            type_name: || type_name::<O>()
        }
    }

}
