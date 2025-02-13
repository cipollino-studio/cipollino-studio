
use std::collections::HashSet;

use crate::{LoadingPtr, Object, Project, Ptr, Recorder, RecreateObjectDelta, Serializable};

use super::{Children, TreeObj};

#[derive(Clone)]
pub struct UnorderedChildList<O: Object> {
    children: HashSet<LoadingPtr<O>>
}

impl<O: Object> UnorderedChildList<O> {

    pub fn new() -> Self {
        Self {
            children: HashSet::new()
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Ptr<O>> + '_ {
        self.children.iter().map(LoadingPtr::ptr)
    }

}

impl<O: Object> Default for UnorderedChildList<O> {

    fn default() -> Self {
        Self::new()
    }

}

impl<O: Object> Serializable<O::Project> for UnorderedChildList<O> {

    fn serialize(&self, context: &crate::SerializationContext<O::Project>) -> rmpv::Value {
        self.children.serialize(context)
    }

    fn deserialize(data: &rmpv::Value, context: &mut crate::DeserializationContext<O::Project>) -> Option<Self> {
        let children = HashSet::<LoadingPtr<O>>::deserialize(data, context)?;
        Some(Self {
            children
        })
    }

}

impl<O: Object> Children<O> for UnorderedChildList<O> {

    type Index = ();

    fn n_children(&self) -> usize {
        self.children.len()
    }

    fn insert(&mut self, _idx: Self::Index, child: Ptr<O>) {
        self.children.insert(LoadingPtr::new(child));
    }

    fn remove(&mut self, child: Ptr<O>) -> Option<Self::Index> {
        if self.children.remove(&LoadingPtr::new(child)) {
            Some(()) 
        } else {
            None
        }
    }

    fn index_of(&self, child: Ptr<O>) -> Option<Self::Index> {
        if self.children.contains(&LoadingPtr::new(child)) {
            Some(())
        } else {
            None
        }
    }


}

pub struct UnorderedChildListTreeData<O: TreeObj> {
    children: Vec<(Ptr<O>, O::TreeData)>
}

impl<O: TreeObj> Default for UnorderedChildListTreeData<O> {

    fn default() -> Self {
        Self { children: Vec::new() }
    }

} 

impl<O: TreeObj> Serializable<<O as Object>::Project> for UnorderedChildListTreeData<O> {

    fn serialize(&self, context: &crate::SerializationContext<<O as Object>::Project>) -> rmpv::Value {
        rmpv::Value::Array(
            self.children.iter()
                .map(|(ptr, obj_data)| rmpv::Value::Array(vec![ptr.serialize(context), obj_data.serialize(context)]))
                .collect()
        )
    }

    fn deserialize(data: &rmpv::Value, context: &mut crate::DeserializationContext<<O as Object>::Project>) -> Option<Self> {
        let data = data.as_array()?;
        let mut children = Vec::new();
        for child in data {
            let Some(child) = child.as_array() else { continue; };
            let Some(ptr_data) = child.get(0) else { continue; };
            let Some(obj_data) = child.get(1) else { continue; };
            let Some(ptr) = Ptr::deserialize(ptr_data, context) else { continue; };
            let Some(obj_data) = O::TreeData::deserialize(obj_data, context) else { continue; };
            children.push((ptr, obj_data));
        }
        Some(Self {
            children,
        })
    }

}

impl<O: TreeObj> UnorderedChildList<O> {

    pub fn collect_data(&self, objects: &<O::Project as Project>::Objects) -> UnorderedChildListTreeData<O> {
        UnorderedChildListTreeData {
            children: self.children.iter()
                .map(|loading_ptr| loading_ptr.ptr())
                .filter_map(|ptr| O::list(objects).get(ptr).map(|obj| (ptr, obj.collect_data(objects))))
                .collect(),
        }
    }

    pub fn destroy(&self, recorder: &mut Recorder<O::Project>) {
        for child in &self.children {
            let ptr = child.ptr();
            if let Some(obj) = recorder.obj_list_mut().delete(ptr) {
                obj.destroy(recorder);
                recorder.push_delta(RecreateObjectDelta {
                    ptr,
                    obj,
                });
            }
        }
    }

}

impl<O: TreeObj> UnorderedChildListTreeData<O> {

    pub fn instance(&self, parent: O::ParentPtr, recorder: &mut crate::Recorder<O::Project>) -> UnorderedChildList<O> {
        for (ptr, obj_data) in &self.children { 
            O::instance(obj_data, *ptr, parent.clone(), recorder);
        }
        UnorderedChildList {
            children: self.children.iter().map(|(ptr, _)| LoadingPtr::new(*ptr)).collect(),
        }
    }

}
