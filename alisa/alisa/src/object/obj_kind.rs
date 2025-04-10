
use std::{any::{type_name, TypeId}, collections::HashSet};

use crate::{Collab, DeserializationContext, File, LoadingPtr, Project, Serializable, SerializationContext};

use super::{ObjRef, Object, Ptr};


pub struct ObjectKind<P: Project> {
    pub(crate) name: &'static str,
    pub(crate) clear_modifications: fn(&mut P::Objects),
    pub(crate) save_modifications: fn(&mut File, objects: &mut P::Objects),
    pub(crate) local_load_objects: fn(&mut File, &mut P::Objects),
    pub(crate) collab_load_objects: fn(&mut P::Objects, &mut Collab<P>),
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
            clear_modifications: |objects| {
                O::list_mut(objects).modified.clear();
                O::list_mut(objects).to_delete.clear();
            },
            save_modifications: |file, objects| {
                for modified in &mut O::list(objects).modified.iter() {
                    if let Some(object) = O::list(objects).get(*modified) {
                        let object_data = object.serialize(&SerializationContext::shallow());
                        if let Some(ptr) = file.get_ptr(modified.key) {
                            file.write(ptr, &object_data);
                        }
                    }
                }
                for deleted in O::list(objects).to_delete.iter() {
                    file.delete(deleted.key);
                }
            },
            local_load_objects: |file, objects| {
                let to_load = std::mem::replace(&mut *O::list_mut(objects).to_load.borrow_mut(), HashSet::new());
                for ptr in to_load {
                    load_object::<O>(file, objects, ptr.key);
                }
            },
            collab_load_objects: |objects, collab| {
                let to_load = std::mem::replace(&mut *O::list_mut(objects).to_load.borrow_mut(), HashSet::new());
                for ptr in to_load {
                    match O::list(objects).get_ref(ptr) {
                        ObjRef::Loading | ObjRef::Loaded(_) => { continue; },
                        _ => {}
                    }
                    O::list_mut(objects).mark_loading(ptr);
                    collab.send_message(rmpv::Value::Map(vec![
                        ("type".into(), "load".into()),
                        ("object".into(), O::NAME.into()),
                        ("key".into(), ptr.key.into()),
                    ]));
                }
            },
            load_object: |file, objects, key| {
                load_object::<O>(file, objects, key);
            },
            load_object_from_message: |objects, key, data| {
                if let Some(obj) = O::deserialize(data, &mut DeserializationContext::collab(objects)) {
                    O::list_mut(objects).insert_loaded(Ptr::from_key(key), obj);
                }
            },
            load_failed: |objects, key| {
                O::list_mut(objects).mark_deleted(Ptr::from_key(key));
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
