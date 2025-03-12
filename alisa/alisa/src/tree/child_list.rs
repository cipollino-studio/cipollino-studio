
use crate::{DeserializationContext, Object, Project, Ptr, Recorder, Serializable, SerializationContext};
use super::{ChildPtr, Children};


#[derive(Clone)]
pub struct ChildList<C: ChildPtr> {
    children: Vec<C>
}

impl<C: ChildPtr> ChildList<C> {

    pub fn new() -> Self {
        Self {
            children: Vec::new()
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = C> + DoubleEndedIterator + '_ {
        self.children.iter().cloned()
    }

    pub fn as_slice(&self) -> &[C] {
        &self.children
    }

}

impl<C: ChildPtr> Default for ChildList<C> {

    fn default() -> Self {
        Self::new()
    }

}

impl<C: ChildPtr> Serializable<C::Project> for ChildList<C> {

    fn serialize(&self, context: &crate::SerializationContext<C::Project>) -> rmpv::Value {
        self.children.serialize(context)
    }

    fn deserialize(data: &rmpv::Value, context: &mut crate::DeserializationContext<C::Project>) -> Option<Self> {
        let children = Vec::<C>::deserialize(data, context)?;
        Some(Self {
            children
        })
    }

}

impl<C: ChildPtr, O: Object> Children<O> for ChildList<C> where C: From<Ptr<O>> + PartialEq + Eq {

    type Index = usize;

    fn n_children(&self) -> usize {
        self.children.len()
    }

    fn insert(&mut self, idx: usize, child: Ptr<O>) {
        let idx = idx.clamp(0, self.n_children());
        self.children.insert(idx, child.into());
    }

    fn remove(&mut self, child: Ptr<O>) -> Option<usize> {
        for i in 0..self.children.len() {
            if self.children[i] == child.into() {
                self.children.remove(i);
                return Some(i);
            }
        }
        None
    }

    fn index_of(&self, child: Ptr<O>) -> Option<usize> {
        for i in 0..self.children.len() {
            if self.children[i] == child.into() {
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

pub struct ChildListTreeData<C: ChildPtr> {
    children: Vec<(C, C::TreeData)>
}

impl<C: ChildPtr> Default for ChildListTreeData<C> {

    fn default() -> Self {
        Self { children: Vec::new() }
    }

} 

impl<C: ChildPtr> Serializable<C::Project> for ChildListTreeData<C> {

    fn serialize(&self, context: &SerializationContext<C::Project>) -> rmpv::Value {
        rmpv::Value::Array(
            self.children.iter()
                .map(|(ptr, obj_data)| rmpv::Value::Array(vec![ptr.serialize(context), obj_data.serialize(context)]))
                .collect()
        )
    }

    fn deserialize(data: &rmpv::Value, context: &mut DeserializationContext<C::Project>) -> Option<Self> {
        let data = data.as_array()?;
        let mut children = Vec::new();
        for child in data {
            let Some(child) = child.as_array() else { continue; };
            let Some(ptr_data) = child.get(0) else { continue; };
            let Some(obj_data) = child.get(1) else { continue; };
            let Some(ptr) = C::deserialize(ptr_data, context) else { continue; };
            let Some(obj_data) = C::TreeData::deserialize(obj_data, context) else { continue; };
            children.push((ptr, obj_data));
        }
        Some(Self {
            children,
        })
    }

}

impl<C: ChildPtr> ChildList<C> {

    pub fn collect_data(&self, objects: &<C::Project as Project>::Objects) -> ChildListTreeData<C> {
        ChildListTreeData {
            children: self.children.iter()
                .filter_map(|ptr| {
                    let data = ptr.collect_data(objects)?;
                    Some((ptr.clone(), data))
                })
                .collect(),
        }
    }

    pub fn destroy(&self, recorder: &mut Recorder<C::Project>) {
        for child in &self.children {
            child.destroy(recorder);
        }
    }

}

impl<C: ChildPtr> ChildListTreeData<C> {

    pub fn instance(&self, parent: C::ParentPtr, recorder: &mut Recorder<C::Project>) -> ChildList<C> {
        for (ptr, obj_data) in &self.children { 
            ptr.instance(obj_data, parent.clone(), recorder);
        }
        ChildList {
            children: self.children.iter().map(|(ptr, _)| ptr.clone()).collect(),
        }
    }

}
