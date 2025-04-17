
use crate::{Ptr, Recorder};

use super::{Children, TreeObj};

pub fn transfer_tree_object<O: TreeObj>(recorder: &mut Recorder<O::Project>, ptr: Ptr<O>, new_parent: &O::ParentPtr, new_idx: &<O::ChildList as Children<O>>::Index) -> bool {

    // Make sure everything we need exists
    let Some(obj) = recorder.get_obj_mut(ptr) else {
        return false;
    };
    let old_parent = obj.parent().clone();
    if O::child_list_mut(old_parent.clone(), recorder).is_none() {
        return false;
    }
    if O::child_list_mut(new_parent.clone(), recorder).is_none() {
        return false;
    }

    // Set the object's parent
    let Some(obj) = recorder.get_obj_mut(ptr) else {
        return false;
    };
    *obj.parent_mut() = new_parent.clone();
    
    // Remove the object from the old parent's child list
    let mut old_idx = None;
    if let Some(old_child_list) = O::child_list_mut(old_parent.clone(), recorder) {
        if let Some(idx) = old_child_list.remove(ptr) {
            old_idx = Some(idx);
        }
    }

    let new_idx = if let Some(old_idx) = old_idx {
        <O::ChildList as Children<O>>::adjust_idx(*new_idx, old_idx)
    } else {
        *new_idx
    };

    // Add the object to the new parent's child list
    if let Some(new_child_list) = O::child_list_mut(new_parent.clone(), recorder) {
        new_child_list.insert(new_idx.clone(), ptr);
    }

    true
}

#[macro_export]
macro_rules! tree_object_transfer_operation {
    ($object: ty) => {
        ::alisa::paste::paste! {

            #[derive(::alisa::Serializable)]
            pub struct [< Transfer $object >] {
                pub ptr: ::alisa::Ptr<$object>,
                pub new_parent: <$object as ::alisa::TreeObj>::ParentPtr,
                pub new_idx: <<$object as ::alisa::TreeObj>::ChildList as ::alisa::Children<$object>>::Index
            }

            impl Default for [< Transfer $object >] {

                fn default() -> Self {
                    Self {
                        ptr: ::alisa::Ptr::null(),
                        new_parent: Default::default(),
                        new_idx: Default::default() 
                    }
                }

            }

            impl ::alisa::Operation for [< Transfer $object >] {

                type Project = <$object as ::alisa::Object>::Project;

                const NAME: &'static str = stringify!([< Transfer $object:camel >]);

                fn perform(&self, recorder: &mut ::alisa::Recorder<Self::Project>) -> bool {
                    ::alisa::transfer_tree_object(recorder, self.ptr, &self.new_parent, &self.new_idx)
                }

            }

            impl ::alisa::InvertibleOperation for [< Transfer $object >] {

                type Inverse = [< Transfer $object:camel >];

                fn inverse(&self, context: &::alisa::ProjectContext<Self::Project>) -> Option<Self::Inverse> {
                    use ::alisa::TreeObj;
                    use ::alisa::Children;
                    let object = context.obj_list().get(self.ptr)?; 
                    let parent = object.parent();
                    let child_list = $object::child_list(parent, context)?; 
                    let idx = child_list.index_of(self.ptr)?;
                    Some(Self {
                        ptr: self.ptr,
                        new_parent: parent,
                        new_idx: <<$object as ::alisa::TreeObj>::ChildList as ::alisa::Children<$object>>::unadjust_idx(idx, self.new_idx)
                    })
                }

            }

        }
    }
}
