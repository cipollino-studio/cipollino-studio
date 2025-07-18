
use crate::{Object, OperationSource, Project, ProjectContext, Ptr, Recorder, Serializable};

mod child_ptr;
pub use child_ptr::*;

mod child_list;
pub use child_list::*;

mod unordered_child_list;
pub use unordered_child_list::*;

/// A list of references to the children of a tree object
pub trait Children<O: Object> {

    type Index: Copy + Serializable + Default + Send + Sync;

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

    fn is_empty(&self) -> bool {
        self.n_children() == 0
    }

}

/// An object that is part of a tree of objects. 
pub trait TreeObj: Object + Send + Sync {

    /// The type of pointer that points to the parent object 
    type ParentPtr: Default + Clone + Send + Sync;
    /// The list of children the parent has that points to this tree object
    type ChildList: Children<Self>;
    /// The information needed to recreate this object and all of its children in the tree
    type TreeData: Serializable;

    /// Get the list of children that points to this object given the parent pointer
    fn child_list<'a>(parent: Self::ParentPtr, context: &'a ProjectContext<Self::Project>) -> Option<&'a Self::ChildList>;
    /// Get a mutable reference to the list of children that points to this object given the parent pointer
    fn child_list_mut<'a>(parent: Self::ParentPtr, recorder: &'a mut Recorder<Self::Project>) -> Option<&'a mut Self::ChildList>;
    /// Get the parent
    fn parent(&self) -> Self::ParentPtr;
    /// Get a mutable reference to the parent
    fn parent_mut(&mut self) -> &mut Self::ParentPtr;

    /// Create this object and all its children from the tree data
    fn instance(data: &Self::TreeData, ptr: Ptr<Self>, parent: Self::ParentPtr, recorder: &mut Recorder<Self::Project>); 
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
