
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

                fn perform(&self, recorder: &mut ::alisa::Recorder<Self::Project>) {
                    use ::alisa::TreeObj;
                    use ::alisa::Children;

                    // Make sure the parent we're creating the object in exists 
                    if $object::child_list_mut(self.parent, recorder.context_mut()).is_none() {
                        return;
                    }

                    // Instance the object and its children
                    $object::instance(&self.data, self.ptr, self.parent, recorder);

                    // Add it to the parent's child list
                    if let Some(child_list) = $object::child_list_mut(self.parent, recorder.context_mut()) {
                        child_list.insert(self.idx, self.ptr);
                        recorder.push_delta(::alisa::RemoveChildDelta {
                            parent: self.parent.clone(),
                            ptr: self.ptr
                        });
                    }
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

                fn perform(&self, recorder: &mut ::alisa::Recorder<Self::Project>) {
                    use ::alisa::Children;
                    use ::alisa::TreeObj;
                    if let Some(obj) = recorder.obj_list_mut().delete(self.ptr) {
                        obj.destroy(recorder);
                        let parent = obj.parent(); 
                        recorder.push_delta(::alisa::RecreateObjectDelta {
                            ptr: self.ptr,
                            obj
                        });
                        if let Some(child_list) = $object::child_list_mut(parent.clone(), recorder.context_mut()) {
                            if let Some(idx) = child_list.remove(self.ptr) {
                                recorder.push_delta(::alisa::InsertChildDelta {
                                    parent,
                                    ptr: self.ptr,
                                    idx
                                });
                            }
                        }
                    }
                }

                fn inverse(&self, context: &::alisa::ProjectContext<Self::Project>) -> Option<Self::Inverse> {
                    use ::alisa::Children;
                    use ::alisa::Object;
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
