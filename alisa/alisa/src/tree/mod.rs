
use crate::{Delta, Object, OperationSource, Project, ProjectContext, ProjectContextMut, Ptr, Recorder, Serializable};

mod child_list;
pub use child_list::*;

mod unordered_child_list;
pub use unordered_child_list::*;

/// A list of references to the children of a tree object
pub trait Children<O: Object> {

    type Index: Copy + Serializable<O::Project> + Default + Send + Sync;

    fn n_children(&self) -> usize;
    fn insert(&mut self, idx: Self::Index, child: Ptr<O>);
    fn remove(&mut self, child: Ptr<O>) -> Option<Self::Index>;
    fn index_of(&self, child: Ptr<O>) -> Option<Self::Index>;

    fn adjust_idx(idx: Self::Index, _removed_idx: Self::Index) -> Self::Index {
        idx
    }

    fn unadjust_idx(idx: Self::Index, _moved_to_idx: Self::Index) -> Self::Index {
        idx
    }

}

pub struct RemoveChildDelta<O: TreeObj> {
    pub parent: O::ParentPtr,
    pub ptr: Ptr<O>
}

impl<O: TreeObj> Delta for RemoveChildDelta<O> {
    type Project = O::Project;

    fn perform(&self, context: &mut crate::ProjectContextMut<'_, Self::Project>) {
        if let Some(list) = O::child_list_mut(self.parent.clone(), context) {
            list.remove(self.ptr);
        }
    }

}

pub struct InsertChildDelta<O: TreeObj> {
    pub parent: O::ParentPtr,
    pub ptr: Ptr<O>,
    pub idx: <O::ChildList as Children<O>>::Index 
}

impl<O: TreeObj> Delta for InsertChildDelta<O> {
    type Project = O::Project;

    fn perform(&self, context: &mut ProjectContextMut<'_, Self::Project>) {
        if let Some(list) = O::child_list_mut(self.parent.clone(), context) {
            list.insert(self.idx, self.ptr);
        }
    }
}

pub struct SetParentDelta<O: TreeObj> {
    pub ptr: Ptr<O>,
    pub new_parent: O::ParentPtr
}

impl<O: TreeObj> Delta for SetParentDelta<O> {
    type Project = O::Project;

    fn perform(&self, context: &mut ProjectContextMut<'_, Self::Project>) {
        if let Some(obj) = context.obj_list_mut().get_mut(self.ptr) {
            *obj.parent_mut() = self.new_parent.clone();
        }
    }
}

/// An object that is part of a tree of objects. 
pub trait TreeObj: Object + Send + Sync {

    /// The type of pointer that points to the parent object 
    type ParentPtr: Default + Clone + Send + Sync;
    /// The list of children the parent has that points to this tree object
    type ChildList: Children<Self>;
    /// The information needed to recreate this object and all of its children in the tree
    type TreeData: Serializable<Self::Project>;

    /// Get the list of children that points to this object given the parent pointer
    fn child_list<'a>(parent: Self::ParentPtr, context: &'a ProjectContext<Self::Project>) -> Option<&'a Self::ChildList>;
    /// Get a mutable reference to the list of children that points to this object given the parent pointer
    fn child_list_mut<'a>(parent: Self::ParentPtr, context: &'a mut ProjectContextMut<Self::Project>) -> Option<&'a mut Self::ChildList>;
    /// Get the parent
    fn parent(&self) -> Self::ParentPtr;
    /// Get a mutable reference to the parent
    fn parent_mut(&mut self) -> &mut Self::ParentPtr;

    /// Create this object and all its children from the tree data
    fn instance(data: &Self::TreeData, ptr: Ptr<Self>, parent: Self::ParentPtr, recorder: &mut Recorder<Self::Project>); 
    /// Delete this object and all its children
    fn destroy(&self, recorder: &mut Recorder<Self::Project>);
    /// Get the tree data for this object and its children
    fn collect_data(&self, objects: &<Self::Project as Project>::Objects) -> Self::TreeData;

    // Can this object be deleted right now?
    fn can_delete(_ptr: Ptr<Self>, _project: &ProjectContext<Self::Project>, _source: OperationSource) -> bool {
        true
    }

}

mod creation;
pub use creation::*;
mod transfer;
pub use transfer::*;

#[macro_export]
macro_rules! tree_object_operations {
    ($object: ty) => {
        ::alisa::tree_object_creation_operations!($object);
        ::alisa::tree_object_transfer_operation!($object);
    };
}
