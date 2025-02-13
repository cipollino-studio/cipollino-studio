
#[macro_export]
macro_rules! tree_object_transfer_operation {
    ($object: ty) => {
        ::alisa::paste::paste! {

            #[derive(::alisa::Serializable)]
            #[project(<$object as ::alisa::Object>::Project)]
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
                type Inverse = [< Transfer $object:camel >];

                const NAME: &'static str = stringify!([< Transfer $object:camel >]);

                fn perform(&self, recorder: &mut ::alisa::Recorder<Self::Project>) {
                    use ::alisa::TreeObj;
                    use ::alisa::Children;

                    // Make sure everything we need exists
                    let Some(obj) = recorder.obj_list_mut().get_mut(self.ptr) else { return; };
                    let old_parent = obj.parent().clone();
                    if $object::child_list_mut(old_parent.clone(), recorder.context_mut()).is_none() {
                        return;
                    }
                    if $object::child_list_mut(self.new_parent.clone(), recorder.context_mut()).is_none() {
                        return;
                    }

                    // Set the object's parent
                    let Some(obj) = recorder.obj_list_mut().get_mut(self.ptr) else { return; };
                    *obj.parent_mut() = self.new_parent.clone();
                    recorder.push_delta(::alisa::SetParentDelta {
                        ptr: self.ptr,
                        new_parent: old_parent.clone()
                    });
                    
                    // Remove the object from the old parent's child list
                    if let Some(old_child_list) = $object::child_list_mut(old_parent.clone(), recorder.context_mut()) {
                        if let Some(idx) = old_child_list.remove(self.ptr) {
                            recorder.push_delta(::alisa::InsertChildDelta {
                                parent: old_parent,
                                ptr: self.ptr,
                                idx
                            });
                        }
                    }

                    // Add the object to the new parent's child list
                    if let Some(new_child_list) = $object::child_list_mut(self.new_parent.clone(), recorder.context_mut()) {
                        new_child_list.insert(self.new_idx, self.ptr);
                        recorder.push_delta(::alisa::RemoveChildDelta {
                            parent: self.new_parent.clone(),
                            ptr: self.ptr
                        });
                    }
                }

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
                        new_idx: idx
                    })
                }

            }

        }
    }
}
