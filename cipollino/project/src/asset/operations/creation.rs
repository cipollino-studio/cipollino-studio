
#[macro_export]
macro_rules! asset_creation_operations {
    ($asset: ty) => {
        alisa::paste::paste! {

            #[derive(alisa::Serializable, Default)]
            #[project(crate::Project)]
            pub struct [< Create $asset:camel >] {
                pub ptr: alisa::Ptr<$asset>,
                pub parent: alisa::Ptr<crate::Folder>,
                pub data: <$asset as alisa::TreeObj>::TreeData
            }

            #[derive(alisa::Serializable, Default)]
            #[project(crate::Project)]
            pub struct [< Delete $asset:camel >] {
                pub ptr: alisa::Ptr<$asset>
            }

            impl alisa::Operation for [< Create $asset:camel >] {

                type Project = crate::Project;
                type Inverse = [< Delete $asset:camel >];
                
                const NAME: &'static str = stringify!([< Create $asset:camel >]);

                fn perform(&self, recorder: &mut alisa::Recorder<crate::Project>) {
                    use alisa::TreeObj;
                    use alisa::Children;

                    // Make sure the parent we're creating the object in exists 
                    let context = &recorder.context();
                    let Some(child_list) = $asset::child_list(self.parent, &context) else { return; };
                    let sibling_names = $asset::get_sibling_names(child_list, recorder.obj_list(), None);

                    // Instance the object and its children
                    $asset::instance(&self.data, self.ptr, self.parent, recorder);

                    crate::rectify_name_duplication(self.ptr, sibling_names, recorder);

                    // Add it to the parent's child list
                    if let Some(child_list) = $asset::child_list_mut(self.parent, recorder.context_mut()) {
                        child_list.insert((), self.ptr);
                        recorder.push_delta(alisa::RemoveChildDelta {
                            parent: self.parent.clone(),
                            ptr: self.ptr
                        });
                    }
                }

                fn inverse(&self, _context: &alisa::ProjectContext<crate::Project>) -> Option<Self::Inverse> {
                    Some(Self::Inverse {
                        ptr: self.ptr
                    })
                }

            }

            impl alisa::Operation for [< Delete $asset:camel >] {

                type Project = crate::Project;
                type Inverse = [< Create $asset:camel >];
                
                const NAME: &'static str = stringify!([< Delete $asset:camel >]);

                fn perform(&self, recorder: &mut alisa::Recorder<crate::Project>) {
                    use ::alisa::Children;
                    use ::alisa::TreeObj;
                    if let Some(obj) = recorder.obj_list_mut().delete(self.ptr) {
                        obj.destroy(recorder);
                        let parent = obj.parent(); 
                        recorder.push_delta(::alisa::RecreateObjectDelta {
                            ptr: self.ptr,
                            obj
                        });
                        if let Some(child_list) = $asset::child_list_mut(parent.clone(), recorder.context_mut()) {
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

                fn inverse(&self, context: &alisa::ProjectContext<crate::Project>) -> Option<Self::Inverse> {
                    use alisa::TreeObj;
                    let object = context.obj_list().get(self.ptr)?; 
                    let data = $asset::collect_data(&object, context.objects());
                    let parent = $asset::parent(&object);
                    Some(Self::Inverse {
                        ptr: self.ptr,
                        parent,
                        data
                    })
                }

            }

        }
    };
}
