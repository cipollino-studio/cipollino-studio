
use crate::{Clip, Objects, Project};

mod layer;
use alisa::{Object, Recorder, TreeObj};
pub use layer::*;

#[derive(Clone, Copy, alisa::Serializable)]
#[project(Project)]
pub enum LayerParent {
    Clip(alisa::Ptr<Clip>)
}

impl Default for LayerParent {
    fn default() -> Self {
        Self::Clip(alisa::Ptr::null())
    }
}

impl LayerParent {

    fn child_list<'a>(&self, context: &'a alisa::ProjectContext<Project>) -> Option<&'a LayerChildList> {
        match self {
            LayerParent::Clip(ptr) => context.obj_list()
                .get(*ptr)
                .and_then(|clip| context.obj_list().get(clip.inner))
                .map(|inner| &inner.layers),
        }
    }

    fn child_list_mut<'a>(&self, recorder: &'a mut alisa::Recorder<Project>) -> Option<&'a mut LayerChildList> {
        match self {
            LayerParent::Clip(ptr) => recorder.get_obj(*ptr)
                .map(|clip| clip.inner)
                .and_then(|inner| recorder.get_obj_mut(inner))
                .map(|inner| &mut inner.layers),
        }
    }

}

#[derive(Clone, Copy, PartialEq, Eq, alisa::Serializable)]
#[project(Project)]
pub enum LayerChildPtr {
    Layer(alisa::LoadingPtr<Layer>)
}

impl LayerChildPtr {

    fn collect_data(&self, objects: &Objects) -> Option<LayerChildTreeData> {
        match self {
            LayerChildPtr::Layer(layer_ptr) => {
                let layer = Layer::list(objects).get(layer_ptr.ptr())?;
                Some(LayerChildTreeData::Layer(layer_ptr.ptr(), layer.collect_data(objects)))
            }
        }
    }

}

#[derive(alisa::Serializable)]
#[project(Project)]
enum LayerChildTreeData {
    Layer(alisa::Ptr<Layer>, <Layer as alisa::TreeObj>::TreeData)
}

trait LayerType: alisa::Object {

    fn make_child_ptr(ptr: alisa::Ptr<Self>) -> LayerChildPtr;

}

#[derive(Default, alisa::Serializable, Clone)]
#[project(Project)]
pub struct LayerChildList {
    children: Vec<LayerChildPtr>
}

impl<L: LayerType> alisa::Children<L> for LayerChildList {
    type Index = usize;

    fn n_children(&self) -> usize {
        self.n_children()
    }

    fn insert(&mut self, idx: usize, child: alisa::Ptr<L>) {
        self.children.insert(idx, L::make_child_ptr(child));
    }

    fn remove(&mut self, child: alisa::Ptr<L>) -> Option<usize> {
        let idx = self.index_of(child)?;
        self.children.remove(idx);
        Some(idx)
    }

    fn index_of(&self, child: alisa::Ptr<L>) -> Option<usize> {
        let child = L::make_child_ptr(child);
        self.children.iter().position(|other| &child == other)
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

impl LayerChildList {

    pub fn n_children(&self) -> usize {
        self.children.len()
    }

    pub(crate) fn collect_data(&self, objects: &Objects) -> LayerChildListTreeData {
        LayerChildListTreeData {
            children: self.children.iter()
                .filter_map(|ptr| ptr.collect_data(objects))
                .collect()
        }
    }

    pub(crate) fn destroy(&self, recorder: &mut Recorder<Project>) {
        for child in &self.children {
            match child {
                LayerChildPtr::Layer(layer_ptr) => {
                    if let Some(layer) = recorder.delete_obj(layer_ptr.ptr()) {
                        layer.destroy(recorder);
                    }
                },
            }
        } 
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = LayerChildPtr> + DoubleEndedIterator + 'a {
        self.children.iter().cloned()
    }

}

#[derive(Default, alisa::Serializable)]
#[project(Project)]
pub struct LayerChildListTreeData {
    children: Vec<LayerChildTreeData>
}

impl LayerChildListTreeData {

    pub(crate) fn instance(&self, parent: LayerParent, recorder: &mut alisa::Recorder<Project>) -> LayerChildList {
        let mut children = Vec::new();
        for child in &self.children {
            match child {
                LayerChildTreeData::Layer(ptr, tree_data) => {
                    Layer::instance(tree_data, *ptr, parent, recorder);
                    children.push(LayerChildPtr::Layer(alisa::LoadingPtr::new(*ptr)));
                },
            }
        }
        LayerChildList {
            children
        }
    }

}
