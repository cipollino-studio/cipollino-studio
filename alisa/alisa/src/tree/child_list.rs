
use crate::{LoadingPtr, Object, Project, Ptr, Recorder, RecreateObjectDelta, Serializable};

use super::{Children, TreeObj};

#[derive(Clone)]
pub struct ChildList<O: Object> {
    children: Vec<LoadingPtr<O>>
}

impl<O: Object> ChildList<O> {

    pub fn new() -> Self {
        Self {
            children: Vec::new()
        }
    }
    
    pub fn iter(&self) -> impl Iterator<Item = Ptr<O>> + '_ {
        self.children.iter().map(LoadingPtr::ptr)
    }

}

impl<O: Object> Default for ChildList<O> {

    fn default() -> Self {
        Self::new()
    }

}

impl<O: Object> Serializable<O::Project> for ChildList<O> {

    fn serialize(&self, context: &crate::SerializationContext<O::Project>) -> rmpv::Value {
        self.children.serialize(context)
    }

    fn deserialize(data: &rmpv::Value, context: &mut crate::DeserializationContext<O::Project>) -> Option<Self> {
        let children = Vec::<LoadingPtr<O>>::deserialize(data, context)?;
        Some(Self {
            children
        })
    }

}

impl<O: Object> Children<O> for ChildList<O> {

    type Index = usize;

    fn n_children(&self) -> usize {
        self.children.len()
    }

    fn insert(&mut self, idx: usize, child: Ptr<O>) {
        let idx = idx.clamp(0, self.n_children());
        self.children.insert(idx, LoadingPtr::new(child));
    }

    fn remove(&mut self, child: Ptr<O>) -> Option<usize> {
        for i in 0..self.children.len() {
            if self.children[i].ptr() == child {
                self.children.remove(i);
                return Some(i);
            }
        }
        None
    }

    fn index_of(&self, child: Ptr<O>) -> Option<usize> {
        for i in 0..self.children.len() {
            if self.children[i].ptr() == child {
                return Some(i);
            }
        }
        None
    }

    fn adjust_idx(idx: usize, removed_idx: usize) -> usize {
        if idx > removed_idx {
            idx - 1
        } else {
            idx
        }
    }

    fn unadjust_idx(idx: usize, moved_to_idx: usize) -> usize {
        if idx > moved_to_idx {
            idx + 1
        } else {
            idx
        }
    }

}

pub struct ChildListTreeData<O: TreeObj> {
    children: Vec<(Ptr<O>, O::TreeData)>
}

impl<O: TreeObj> Default for ChildListTreeData<O> {

    fn default() -> Self {
        Self { children: Vec::new() }
    }

} 

impl<O: TreeObj> Serializable<<O as Object>::Project> for ChildListTreeData<O> {

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

impl<O: TreeObj> ChildList<O> {

    pub fn collect_data(&self, objects: &<O::Project as Project>::Objects) -> ChildListTreeData<O> {
        ChildListTreeData {
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

impl<O: TreeObj> ChildListTreeData<O> {

    pub fn instance(&self, parent: O::ParentPtr, recorder: &mut crate::Recorder<O::Project>) -> ChildList<O> {
        for (ptr, obj_data) in &self.children { 
            O::instance(obj_data, *ptr, parent.clone(), recorder);
        }
        ChildList {
            children: self.children.iter().map(|(ptr, _)| LoadingPtr::new(*ptr)).collect(),
        }
    }

}
