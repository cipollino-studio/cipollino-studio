
use crate::{Ptr, Recorder};

use super::{Children, TreeObj};

pub fn create_tree_object<O: TreeObj>(recorder: &mut Recorder<O::Project>, ptr: Ptr<O>, parent: O::ParentPtr, idx: <O::ChildList as Children<O>>::Index, data: &O::TreeData) -> bool {
    // Make sure the parent we're creating the object in exists 
    if O::child_list_mut(parent.clone(), recorder).is_none() {
        return false;
    }

    // Instance the object and its children
    O::instance(data, ptr, parent.clone(), recorder);

    // Add it to the parent's child list
    let Some(child_list) = O::child_list_mut(parent.clone(), recorder) else {
        return false;
    };
    child_list.insert(idx, ptr);

    true
}

pub fn delete_tree_object<O: TreeObj>(recorder: &mut Recorder<O::Project>, ptr: Ptr<O>) -> bool {
    if let Some(obj) = recorder.delete_obj(ptr) {
        obj.destroy(recorder);
        let parent = obj.parent(); 
        if let Some(child_list) = O::child_list_mut(parent.clone(), recorder) {
            if let Some(_idx) = child_list.remove(ptr) {
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

                const NAME: &'static str = stringify!([< Create $object:camel >]);

                fn perform(&self, recorder: &mut ::alisa::Recorder<Self::Project>) -> bool {
                    ::alisa::create_tree_object(recorder, self.ptr, self.parent.clone(), self.idx.clone(), &self.data)
                }


            }

            impl ::alisa::InvertibleOperation for [< Create $object:camel >] {

                type Inverse = [< Delete $object:camel >];

                fn inverse(&self, context: &::alisa::ProjectContext<Self::Project>) -> Option<Self::Inverse> {
                    Some([<Delete $object:camel >] {
                        ptr: self.ptr
                    })
                }

            }

            #[derive(::alisa::Serializable)]
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

                const NAME: &'static str = stringify!([< Delete $object:camel >]);

                fn perform(&self, recorder: &mut ::alisa::Recorder<Self::Project>) -> bool {
                    use ::alisa::TreeObj;
                    if !$object::can_delete(self.ptr, &recorder.context(), recorder.source()) {
                        return false;
                    }
                    ::alisa::delete_tree_object(recorder, self.ptr)
                }


            }

            impl ::alisa::InvertibleOperation for [< Delete $object:camel >] {

                type Inverse = [< Create $object:camel >];

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
