
use crate::{Ptr, Recorder, RecreateObjectDelta};

use super::{Children, InsertChildDelta, RemoveChildDelta, TreeObj};

pub fn create_tree_object<O: TreeObj>(recorder: &mut Recorder<O::Project>, ptr: Ptr<O>, parent: O::ParentPtr, idx: <O::ChildList as Children<O>>::Index, data: &O::TreeData) -> bool {
    // Make sure the parent we're creating the object in exists 
    if O::child_list_mut(parent.clone(), recorder.context_mut()).is_none() {
        return false;
    }

    // Instance the object and its children
    O::instance(data, ptr, parent.clone(), recorder);

    // Add it to the parent's child list
    let Some(child_list) = O::child_list_mut(parent.clone(), recorder.context_mut()) else {
        return false;
    };
    child_list.insert(idx, ptr);
    recorder.push_delta(RemoveChildDelta {
        parent: parent.clone(),
        ptr: ptr
    });

    true
}

pub fn delete_tree_object<O: TreeObj>(recorder: &mut Recorder<O::Project>, ptr: Ptr<O>) -> bool {
    if let Some(obj) = recorder.obj_list_mut().delete(ptr) {
        obj.destroy(recorder);
        let parent = obj.parent(); 
        recorder.push_delta(RecreateObjectDelta {
            ptr,
            obj
        });
        if let Some(child_list) = O::child_list_mut(parent.clone(), recorder.context_mut()) {
            if let Some(idx) = child_list.remove(ptr) {
                recorder.push_delta(InsertChildDelta {
                    parent,
                    ptr,
                    idx
                });
                return true;
            }
        }
    }
    false 
}

#[macro_export]
macro_rules! tree_object_creation_operations {
    ($object: ty) => {
        ::alisa::paste::paste! {

            #[derive(::alisa::Serializable)]
            #[project(<$object as ::alisa::Object>::Project)]
            pub struct [< Create $object:camel >] {
                pub ptr: ::alisa::Ptr<$object>,
                pub parent: <$object as ::alisa::TreeObj>::ParentPtr,
                pub idx: <<$object as ::alisa::TreeObj>::ChildList as ::alisa::Children<$object>>::Index,
                pub data: <$object as ::alisa::TreeObj>::TreeData
            }

            impl Default for [< Create $object:camel >] {

                fn default() -> Self {
                    Self {
                        ptr: ::alisa::Ptr::null(),
                        parent: Default::default(),
                        idx: Default::default(),
                        data: <<$object as ::alisa::TreeObj>::TreeData as Default>::default()
                    }
                }

            }

            impl ::alisa::Operation for [< Create $object:camel >] {

                type Project = <$object as ::alisa::Object>::Project;
                type Inverse = [< Delete $object:camel >];

                const NAME: &'static str = stringify!([< Create $object:camel >]);

                fn perform(&self, recorder: &mut ::alisa::Recorder<Self::Project>) -> bool {
                    ::alisa::create_tree_object(recorder, self.ptr, self.parent.clone(), self.idx.clone(), &self.data)
                }

                fn inverse(&self, context: &::alisa::ProjectContext<Self::Project>) -> Option<Self::Inverse> {
                    Some([<Delete $object:camel >] {
                        ptr: self.ptr
                    })
                }

            }

            #[derive(::alisa::Serializable)]
            #[project(<$object as ::alisa::Object>::Project)]
            pub struct [< Delete $object:camel >] {
                pub ptr: ::alisa::Ptr<$object>
            }

            impl Default for [< Delete $object:camel >] {

                fn default() -> Self {
                    Self {
                        ptr: ::alisa::Ptr::null()
                    }
                }

            } 

            impl ::alisa::Operation for [< Delete $object:camel >] {

                type Project = <$object as ::alisa::Object>::Project;
                type Inverse = [< Create $object:camel >];

                const NAME: &'static str = stringify!([< Delete $object:camel >]);

                fn perform(&self, recorder: &mut ::alisa::Recorder<Self::Project>) -> bool {
                    use ::alisa::TreeObj;
                    if !$object::can_delete(self.ptr, &recorder.context(), recorder.source()) {
                        return false;
                    }
                    ::alisa::delete_tree_object(recorder, self.ptr)
                }

                fn inverse(&self, context: &::alisa::ProjectContext<Self::Project>) -> Option<Self::Inverse> {
                    use ::alisa::Children;
                    use ::alisa::Object;
                    use ::alisa::TreeObj;
                    if !$object::can_delete(self.ptr, context, ::alisa::OperationSource::Local) {
                        return None;
                    }
                    let object = context.obj_list().get(self.ptr)?; 
                    let data = <$object as ::alisa::TreeObj>::collect_data(&object, context.objects());
                    let parent = <$object as ::alisa::TreeObj>::parent(&object);
                    let child_list = <$object as ::alisa::TreeObj>::child_list(parent, context)?;
                    let idx = child_list.index_of(self.ptr)?;
                    Some([< Create $object:camel >] {
                        ptr: self.ptr,
                        idx,
                        parent,
                        data
                    })
                }

            }


        }
    }
}
