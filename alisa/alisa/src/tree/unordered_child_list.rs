
use std::collections::HashSet;
use std::hash::Hash;

use crate::{ABFValue, DeserializationContext, Object, Project, Ptr, Recorder, Serializable, SerializationContext};

use super::{ChildPtr, Children};

#[derive(Clone)]
pub struct UnorderedChildList<C: ChildPtr + Hash> {
    children: HashSet<C>
}

impl<C: ChildPtr + Hash> UnorderedChildList<C> {

    pub fn new() -> Self {
        Self {
            children: HashSet::new()
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = C> + '_ {
        self.children.iter().cloned()
    }

}

impl<C: ChildPtr + Hash> Default for UnorderedChildList<C> {

    fn default() -> Self {
        Self::new()
    }

}

impl<C: ChildPtr + Hash> Serializable for UnorderedChildList<C> {

    fn serialize(&self, context: &SerializationContext) -> ABFValue {
        self.children.serialize(context)
    }

    fn deserialize(data: &ABFValue, context: &mut DeserializationContext) -> Option<Self> {
        let children = HashSet::<C>::deserialize(data, context)?;
        Some(Self {
            children
        })
    }

}

impl<C: ChildPtr + Hash, O: Object> Children<O> for UnorderedChildList<C> where C: From<Ptr<O>> {

    type Index = ();

    fn n_children(&self) -> usize {
        self.children.len()
    }

    fn insert(&mut self, _idx: Self::Index, child: Ptr<O>) {
        self.children.insert(child.into());
    }

    fn remove(&mut self, child: Ptr<O>) -> Option<Self::Index> {
        if self.children.remove(&child.into()) {
            Some(()) 
        } else {
            None
        }
    }

    fn index_of(&self, child: Ptr<O>) -> Option<Self::Index> {
        if self.children.contains(&child.into()) {
            Some(())
        } else {
            None
        }
    }


}

pub struct UnorderedChildListTreeData<C: ChildPtr> {
    pub children: Vec<(C, C::TreeData)>
}

impl<C: ChildPtr> Default for UnorderedChildListTreeData<C> {

    fn default() -> Self {
        Self { children: Vec::new() }
    }

} 

impl<C: ChildPtr + Hash> Serializable for UnorderedChildListTreeData<C> {

    fn serialize(&self, context: &crate::SerializationContext) -> ABFValue {
        ABFValue::Array(
            self.children.iter()
                .map(|(ptr, obj_data)| ABFValue::Array(Box::new([ptr.serialize(context), obj_data.serialize(context)])))
                .collect()
        )
    }

    fn deserialize(data: &ABFValue, context: &mut crate::DeserializationContext) -> Option<Self> {
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

impl<C: ChildPtr + Hash> UnorderedChildList<C> {

    pub fn collect_data(&self, objects: &<C::Project as Project>::Objects) -> UnorderedChildListTreeData<C> {
        UnorderedChildListTreeData {
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

impl<C: ChildPtr + Hash> UnorderedChildListTreeData<C> {

    pub fn instance(&self, parent: C::ParentPtr, recorder: &mut Recorder<C::Project>) -> UnorderedChildList<C> {
        for (ptr, obj_data) in &self.children { 
            ptr.instance(obj_data, parent.clone(), recorder);
        }
        UnorderedChildList {
            children: self.children.iter().map(|(ptr, _)| ptr.clone()).collect(),
        }
    }

}
