use std::hash::Hash;

use crate::{Object, Ptr};

use super::{DeserializationContext, DeserializationContextKind, Serializable, SerializationContext, SerializationContextKind};

const ALREADY_ENCODED_MSGPACK_EXT_CODE: i8 = 123;
const ALREADY_ENCODED_MSGPACK_EXT_DATA: &'static [u8] = b"ENCODED";

/// A reference to an object that indicates that the object refered to should be loaded from disk/the server when the referer is loaded. 
pub struct LoadingPtr<O: Object> {
    ptr: Ptr<O>
}

impl<O: Object> LoadingPtr<O> {

    pub fn new(ptr: Ptr<O>) -> Self {
        Self {
            ptr
        }
    }

    pub fn ptr(&self) -> Ptr<O> {
        self.ptr
    }

    fn load_from_key_and_data(key: u64, object_data: &rmpv::Value, context: &mut DeserializationContext<O::Project>) -> Option<Self> {
        let ptr = Ptr::from_key(key);
        if !matches!(object_data.as_ext(), Some((ALREADY_ENCODED_MSGPACK_EXT_CODE, _))) {
            let object = O::deserialize(&object_data, context)?;
            match &mut context.kind {
                DeserializationContextKind::Local { file: _, objects } | 
                DeserializationContextKind::Collab { objects } => {
                    O::list_mut(objects).insert(ptr, object);
                },
                DeserializationContextKind::Data => unreachable!(),
            }
        }
        Some(LoadingPtr { ptr })
    } 

}

impl<O: Object> Serializable<O::Project> for LoadingPtr<O> {

    fn deserialize(data: &rmpv::Value, context: &mut DeserializationContext<O::Project>) -> Option<Self> {
        match &mut context.kind {
            DeserializationContextKind::Local { file, objects } => {
                let key = data.as_u64()?;
                let ptr = Ptr::from_key(key);

                // If the object is already loaded, skip loading it
                if O::list(objects).get(ptr).is_some() || context.loaded.contains(&key) {
                    return Some(Self {
                        ptr
                    });
                }
                context.loaded.insert(key);

                let file_ptr = file.get_ptr(key)?;
                let object_data = file.read(file_ptr)?; 

                Self::load_from_key_and_data(key, &object_data, context)
            },
            DeserializationContextKind::Collab { objects } => {
                let data = data.as_array()?;
                let key = data.get(0)?.as_u64()?;
                let ptr = Ptr::from_key(key);
                let object_data = data.get(1)?;

                // If the object data is encoded elsewhere in the message, just return the pointer
                if object_data.is_ext() && object_data.as_ext()?.0 == ALREADY_ENCODED_MSGPACK_EXT_CODE && object_data.as_ext()?.1 == ALREADY_ENCODED_MSGPACK_EXT_DATA {
                    return Some(LoadingPtr { ptr })
                } 

                // If the object is already loaded, skip loading it
                if O::list(objects).get(ptr).is_some() || context.loaded.contains(&key) {
                    return Some(Self {
                        ptr
                    });
                }
                context.loaded.insert(key);

                Self::load_from_key_and_data(key, object_data, context)
            },
            DeserializationContextKind::Data => {
                todo!()
            }
        }
    }

    fn serialize(&self, context: &SerializationContext<O::Project>) -> rmpv::Value {
        match &context.kind {
            SerializationContextKind::Shallow => {
                self.ptr.key.into()
            },
            SerializationContextKind::Deep { objects } => {
                // If we already encoded this object somewhere in the given MessagePack value, return
                if context.stored.borrow().contains(&self.ptr.key) {
                    return rmpv::Value::Array(vec![
                        self.ptr.key.into(),
                        rmpv::Value::Ext(ALREADY_ENCODED_MSGPACK_EXT_CODE, ALREADY_ENCODED_MSGPACK_EXT_DATA.into())
                    ]);
                } 
                context.stored.borrow_mut().insert(self.ptr.key);

                let obj_data = O::list(objects).get(self.ptr).map(|obj| obj.serialize(context)).unwrap_or(rmpv::Value::Nil);
                rmpv::Value::Array(vec![
                    self.ptr.key.into(),
                    obj_data
                ])
            }
        }
    }

}

impl<O: Object> Clone for LoadingPtr<O> {

    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr
        }
    }

}

impl<O: Object> Copy for LoadingPtr<O> {}

impl<O: Object> PartialEq for LoadingPtr<O> {

    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }

}

impl<O: Object> Eq for LoadingPtr<O> {}

impl<O: Object> Hash for LoadingPtr<O> {

    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }

}
