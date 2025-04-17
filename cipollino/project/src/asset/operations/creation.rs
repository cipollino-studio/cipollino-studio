
#[macro_export]
macro_rules! asset_creation_operations {
    ($asset: ty) => {
        alisa::paste::paste! {

            #[derive(alisa::Serializable, Default)]
            pub struct [< Create $asset:camel >] {
                pub ptr: alisa::Ptr<$asset>,
                pub parent: alisa::Ptr<crate::Folder>,
                pub data: <$asset as alisa::TreeObj>::TreeData
            }

            #[derive(alisa::Serializable, Default)]
            pub struct [< Delete $asset:camel >] {
                pub ptr: alisa::Ptr<$asset>
            }

            impl alisa::Operation for [< Create $asset:camel >] {

                type Project = crate::Project;
                
                const NAME: &'static str = stringify!([< Create $asset:camel >]);

                fn perform(&self, recorder: &mut alisa::Recorder<crate::Project>) -> bool {
                    use alisa::TreeObj;
                    use alisa::Children;

                    // Make sure the parent we're creating the object in exists 
                    let context = &recorder.context();
                    let Some(child_list) = $asset::child_list(self.parent, &context) else {
                        return false;
                    };
                    let sibling_names = $asset::get_sibling_names(child_list, recorder, None);

                    // Instance the object and its children
                    $asset::instance(&self.data, self.ptr, self.parent, recorder);

                    crate::rectify_name_duplication(self.ptr, sibling_names, recorder);

                    // Add it to the parent's child list
                    let Some(child_list) = $asset::child_list_mut(self.parent, recorder) else {
                        return false;
                    };

                    child_list.insert((), self.ptr);

                    true
                }


            }

            impl alisa::InvertibleOperation for [< Create $asset:camel >] {

                type Inverse = [< Delete $asset:camel >];

                fn inverse(&self, _context: &alisa::ProjectContext<crate::Project>) -> Option<Self::Inverse> {
                    Some(Self::Inverse {
                        ptr: self.ptr
                    })
                }

            }

            impl alisa::Operation for [< Delete $asset:camel >] {

                type Project = crate::Project;

                const NAME: &'static str = stringify!([< Delete $asset:camel >]);

                fn perform(&self, recorder: &mut alisa::Recorder<crate::Project>) -> bool {
                    use ::alisa::TreeObj;
                    if !$asset::can_delete(self.ptr, &recorder.context(), recorder.source()) {
                        return false;
                    }
                    alisa::delete_tree_object(recorder, self.ptr) 
                }

            }

            impl alisa::InvertibleOperation for [< Delete $asset:camel >] {

                type Inverse = [< Create $asset:camel >];

                fn inverse(&self, context: &alisa::ProjectContext<crate::Project>) -> Option<Self::Inverse> {
                    use alisa::TreeObj;
                    if !$asset::can_delete(self.ptr, context, alisa::OperationSource::Local) {
                        return None;
                    }
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
