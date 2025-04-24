
use std::{any::{type_name, TypeId}, collections::HashSet};

use crate::{ABFValue, Collab, DeserializationContext, File, Message, Project, SerializationContext};

use super::{ObjRef, Object, Ptr};

pub struct ObjectKind<P: Project> {
    pub(crate) name: &'static str,
    pub(crate) object_type_id: u16,
    pub(crate) clear_modifications: fn(&mut P::Objects),
    pub(crate) clear_user_modified: fn(&mut P::Objects),
    pub(crate) save_modifications: fn(&mut File, objects: &mut P::Objects),
    pub(crate) local_load_objects: fn(&mut File, &mut P::Objects),
    pub(crate) collab_load_objects: fn(&mut P::Objects, &mut Collab<P>),
    pub(crate) load_object: fn(&mut File, &mut P::Objects, u64),
    pub(crate) load_object_from_message: fn(&mut P::Objects, u64, &ABFValue),
    pub(crate) load_failed: fn(&mut P::Objects, u64),
    pub(crate) serialize_object: fn(&mut P::Objects, u64, &SerializationContext) -> Option<ABFValue>,

    #[cfg(debug_assertions)]
    pub(crate) type_id: fn() -> TypeId,
    #[cfg(debug_assertions)]
    pub(crate) type_name: fn() -> &'static str
}

fn load_object<O: Object>(file: &mut File, objects: &mut <O::Project as Project>::Objects, key: u64) {
    let ptr = Ptr::from_key(key);

    // If the object is already loaded, skip loading it
    if O::list(objects).get(ptr).is_some() {
        return;
    }

    let Some(file_ptr) = file.get_ptr(key) else {
        O::list_mut(objects).mark_deleted(ptr);
        return;
    };
    let Some(object_data) = file.read(file_ptr) else {
        O::list_mut(objects).mark_deleted(ptr);
        return;
    };
    let mut context = DeserializationContext::new();
    let Some(object) = O::deserialize(&object_data, &mut context) else {
        O::list_mut(objects).mark_deleted(ptr);
        return;
    };

    file.load_requested_objects::<O::Project>(context.load_requests, objects);

    O::list_mut(objects).insert_loaded(ptr, object);
}

impl<P: Project> ObjectKind<P> {

    pub const fn from<O: Object<Project = P>>() -> Self {
        Self {
            name: O::NAME,
            object_type_id: O::TYPE_ID,
            clear_modifications: |objects| {
                O::list_mut(objects).modified.clear();
                O::list_mut(objects).to_delete.clear();
            },
            clear_user_modified: |objects| {
                O::list_mut(objects).user_modified.clear();
            },
            save_modifications: |file, objects| {
                for modified in &mut O::list(objects).modified.iter() {
                    if let Some(object) = O::list(objects).get(*modified) {
                        let object_data = object.serialize(&SerializationContext::new());
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
                    collab.send_message(Message::LoadRequest {
                        ptr: ptr.any(),
                    });
                }
            },
            load_object: |file, objects, key| {
                load_object::<O>(file, objects, key);
            },
            load_object_from_message: |objects, key, data| {
                if let Some(obj) = O::deserialize(data, &mut DeserializationContext::new()) {
                    O::list_mut(objects).insert_loaded(Ptr::from_key(key), obj);
                }
            },
            load_failed: |objects, key| {
                O::list_mut(objects).mark_deleted(Ptr::from_key(key));
            },
            serialize_object: |objects, key, context| {
                O::list(objects).get(Ptr::from_key(key)).map(|data| data.serialize(context))
            },
            #[cfg(debug_assertions)]
            type_id: || TypeId::of::<O>(),
            #[cfg(debug_assertions)]
            type_name: || type_name::<O>()
        }
    }

}
