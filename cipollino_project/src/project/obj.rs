
use std::collections::HashMap;
use std::marker::PhantomData;
use std::hash::Hash;
use std::ops::Deref;

use crate::crdt::fractional_index::FractionalIndex;
use crate::crdt::register::Register;
use crate::serialization::{ObjSerialize, Serializer};

use super::Project;

pub trait Obj: Sized + ObjSerialize {

    type Parent;

    fn obj_list(project: &Project) -> &ObjList<Self>;
    fn obj_list_mut(project: &mut Project) -> &mut ObjList<Self>;

    fn parent(&self) -> &Register<(Self::Parent, FractionalIndex)>;
    fn parent_mut(&mut self) -> &mut Register<(Self::Parent, FractionalIndex)>;

    fn list_in_parent(project: &Project, parent: Self::Parent) -> Option<&ChildList<Self>>;
    fn list_in_parent_mut(project: &mut Project, parent: Self::Parent) -> Option<&mut ChildList<Self>>;

}

pub struct ObjPtr<T: Obj> {
    pub(crate) key: u64,
    _marker: PhantomData<T>
}

impl<T: Obj> ObjPtr<T> {

    pub fn from_key(key: u64) -> Self {
        Self {
            key,
            _marker: PhantomData
        }
    }

    pub fn null() -> Self {
        Self::from_key(0) 
    }

}

impl<T: Obj> serde::Serialize for ObjPtr<T> {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_u64(self.key)
    }

}

impl<'de, T: Obj> serde::Deserialize<'de> for ObjPtr<T> {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        u64::deserialize(deserializer).map(|key| ObjPtr {
            key,
            _marker: PhantomData
        })
    }

}

impl<T: Obj> ObjSerialize for ObjPtr<T> {

    fn obj_serialize(&self, _project: &Project, serializer: &mut Serializer) -> bson::Bson {
        let mut fields = bson::doc! {
            "key": self.key as i64
        };

        if let Some(ptr) = serializer.obj_ptrs.get(&self.key) {
            fields.insert("ptr", bson::Bson::Int64(*ptr as i64));
        } 

        bson::Bson::Document(fields)
    }

    fn obj_deserialize(_project: &mut Project, data: &bson::Bson, serializer: &mut Serializer, _idx: FractionalIndex) -> Option<Self> {
        let doc = data.as_document()?;
        let key = doc.get("key")?.as_i64()?;
        let ptr = doc.get("ptr")?.as_i64()?;

        if key <= 0 || ptr <= 0 {
            return None;
        }

        let key = key as u64;
        let ptr = ptr as u64;

        serializer.obj_ptrs.insert(key, ptr);

        Some(Self {
            key: key,
            _marker: PhantomData 
        })
    }

}

impl<T: Obj> Clone for ObjPtr<T> {

    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            _marker: PhantomData
        }
    }

}

impl<T: Obj> Copy for ObjPtr<T> {}

impl<T: Obj> PartialEq for ObjPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self._marker == other._marker
    }


}

impl<T: Obj> Eq for ObjPtr<T> {}

impl<T: Obj> Hash for ObjPtr<T> {

    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }

}

pub struct ObjList<T: Obj> {
    pub(crate) objs: HashMap<ObjPtr<T>, T>,
}

impl<T: Obj> ObjList<T> {

    pub(crate) fn new() -> Self {
        Self {
            objs: HashMap::new(),
        }
    }

    pub fn get(&self, ptr: ObjPtr<T>) -> Option<&T> {
        self.objs.get(&ptr)
    }
    
    pub(crate) fn get_mut(&mut self, ptr: ObjPtr<T>) -> Option<&mut T> {
        self.objs.get_mut(&ptr)
    }

}

pub struct ObjRef<'a, T: Obj> {
    pub(crate) ptr: ObjPtr<T>,
    pub(crate) obj: &'a T 
}

impl<T: Obj> ObjRef<'_, T> {

    pub fn ptr(&self) -> ObjPtr<T> {
        self.ptr
    }

}

impl<T: Obj> Deref for ObjRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.obj
    }
}

/**
    A list of children objects. 
    Essentially a LSEQ CRDT.
 */
pub struct ChildList<T: Obj> {
    pub objs: Vec<(FractionalIndex, ObjPtr<T>)>
}

impl<T: Obj> ChildList<T> {

    pub fn new() -> Self {
        Self {
            objs: Vec::new()
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = ObjPtr<T>> + '_ + Clone {
        self.objs.iter().map(|(_, ptr)| *ptr)
    }

    pub fn iter_ref<'a>(&'a self, obj_list: &'a ObjList<T>) -> impl Iterator<Item = ObjRef<'a, T>> + '_ + Clone {
        self.objs.iter().map(|(_, ptr)| ObjRef {
            ptr: *ptr,
            obj: obj_list.get(*ptr).expect("child obj missing.")
        })
    }

    pub(crate) fn remove(&mut self, ptr: ObjPtr<T>) {
        self.objs.retain(|(_idx, other)| *other != ptr);
    }

    pub(crate) fn insert(&mut self, idx: FractionalIndex, ptr: ObjPtr<T>) {
        let arr_idx = self.objs.binary_search_by(|other_idx| other_idx.0.cmp(&idx)).expect_err("LSEQ keys must be unique.");
        self.objs.insert(arr_idx, (idx, ptr));
    }

    /**
        Get the fractional index you need to insert at to put a new object at a certain index
     */
    pub(crate) fn get_insertion_idx(&self, idx: usize) -> FractionalIndex {
        if self.objs.is_empty() {
            FractionalIndex::half()
        } else if idx == 0 {
            self.objs[0].0.avg_with_0()
        } else if idx >= self.objs.len() {
            self.objs[self.objs.len() - 1].0.avg_with_1()
        } else {
            FractionalIndex::avg(&self.objs[idx - 1].0, &self.objs[idx].0)
        }
    }

}

impl<T: Obj> ObjSerialize for ChildList<T> {

    fn obj_serialize(&self, project: &Project, serializer: &mut Serializer) -> bson::Bson {
        bson::Bson::Array(self.objs.iter().map(|(_idx, ptr)| ptr.obj_serialize(project, serializer)).collect())
    }

    fn obj_deserialize(project: &mut Project, data: &bson::Bson, serializer: &mut Serializer, idx: FractionalIndex) -> Option<Self> {

        let ptrs = data.as_array()?.iter().filter_map(|ptr_data| ObjPtr::obj_deserialize(project, ptr_data, serializer, idx.clone())).collect::<Vec<ObjPtr<T>>>();
        let idxs = FractionalIndex::range(ptrs.len());

        let mut idx_ptr_list: Vec<_> = idxs.into_iter().zip(ptrs.into_iter()).collect();

        idx_ptr_list.retain(|(idx, ptr)| {
            let Some(ptr_in_file) = serializer.obj_ptrs.get(&ptr.key) else { return false; };
            let ptr_in_file = *ptr_in_file;

            let Some(data) = serializer.project_file.get_obj_data(ptr_in_file).ok() else { return false; };
            let Some(obj) = T::obj_deserialize(project, &data, serializer, idx.clone()) else { return false; };

            T::obj_list_mut(project).objs.insert(*ptr, obj);

            true
        });

        Some(Self {
            objs: idx_ptr_list
        })
    }

}
