
#[macro_export]
macro_rules! asset_rename_operation {
    ($asset: ty) => {
        alisa::paste::paste! {

            #[derive(alisa::Serializable)]
            #[project(crate::Project)]
            pub struct [< Rename $asset:camel >] {
                pub ptr: alisa::Ptr<$asset>,
                pub name: String
            }

            impl Default for [< Rename $asset:camel >] {

                fn default() -> Self {
                    Self {
                        ptr: alisa::Ptr::null(),
                        name: stringify!($asset).to_owned()
                    }
                }

            }

            impl alisa::Operation for [< Rename $asset:camel >] {

                type Project = crate::Project;
                type Inverse = [< Rename $asset:camel >];
                
                const NAME: &'static str = stringify!([< Rename $asset:camel >]);

                fn perform(&self, recorder: &mut alisa::Recorder<Self::Project>) {
                    use alisa::TreeObj; 
                    let Some(obj) = recorder.obj_list().get(self.ptr) else { return; };
                    let context = recorder.context();
                    let Some(child_list) = $asset::child_list(obj.parent(), &context) else { return; };
                    let sibling_names = $asset::get_sibling_names(child_list, recorder.obj_list(), Some(self.ptr));

                    if let Some(obj) = recorder.obj_list_mut().get_mut(self.ptr) {
                        let old_name = obj.name().clone();
                        *obj.name_mut() = self.name.clone();

                        crate::rectify_name_duplication(self.ptr, sibling_names, recorder);

                        recorder.push_delta(crate::SetAssetNameDelta {
                            ptr: self.ptr,
                            name: old_name
                        });
                    }
                }

                fn inverse(&self, context: &alisa::ProjectContext<Self::Project>) -> Option<Self::Inverse> {
                    let object = context.obj_list().get(self.ptr)?; 
                    Some(Self {
                        ptr: self.ptr,
                        name: object.name().clone()
                    })
                }

            }

        }
    }
}
