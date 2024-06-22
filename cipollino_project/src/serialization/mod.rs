
use std::collections::HashMap;

use project_file::ProjectFile;

use crate::{crdt::register::Register, project::Project};

pub mod project_file;

pub struct Serializer {
    pub(crate) obj_ptrs: HashMap<u64, u64>,
    pub(crate) project_file: ProjectFile
}

impl Serializer {

    pub(crate) fn get_ptr(&mut self, key: u64) -> Option<u64> {
        if let Some(ptr) = self.obj_ptrs.get(&key) {
            Some(*ptr)
        } else {
            let ptr = self.project_file.alloc_page().ok()?;
            self.obj_ptrs.insert(key, ptr);
            Some(ptr)
        }
    }

}

pub trait ObjSerialize: Sized {

    fn obj_serialize(&self, project: &Project, serializer: &mut Serializer) -> bson::Bson; 
    fn obj_deserialize(project: &mut Project, data: &bson::Bson, serializer: &mut Serializer) -> Option<Self>;

}

impl ObjSerialize for String {

    fn obj_serialize(&self, _project: &Project, _serializer: &mut Serializer) -> bson::Bson {
        bson::Bson::String(self.clone())
    }

    fn obj_deserialize(_project: &mut Project, data: &bson::Bson, _serializer: &mut Serializer) -> Option<Self> {
        Some(data.as_str()?.to_owned()) 
    }

}

impl<T: ObjSerialize> ObjSerialize for Register<T> {

    fn obj_serialize(&self, project: &Project, serialization: &mut Serializer) -> bson::Bson {
        self.value.obj_serialize(project, serialization)
    }

    fn obj_deserialize(project: &mut Project, data: &bson::Bson, serializer: &mut Serializer) -> Option<Self> {
        Some(Self::new(T::obj_deserialize(project, data, serializer)?, 0))
    }

}
