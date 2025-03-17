
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
                
                const NAME: &'static str = stringify!([< Rename $asset:camel >]);

                fn perform(&self, recorder: &mut alisa::Recorder<Self::Project>) -> bool {
                    use alisa::TreeObj; 
                    let Some(obj) = recorder.obj_list().get(self.ptr) else {
                        return false;
                    };
                    let context = recorder.context();
                    let Some(child_list) = $asset::child_list(obj.parent(), &context) else {
                        return false;
                    };
                    let sibling_names = $asset::get_sibling_names(child_list, recorder.obj_list(), Some(self.ptr));

                    let Some(obj) = recorder.get_obj_mut(self.ptr) else {
                        return false;
                    };

                    *obj.name_mut() = self.name.clone();

                    crate::rectify_name_duplication(self.ptr, sibling_names, recorder);

                    true
                }

            }

            impl alisa::InvertibleOperation for [< Rename $asset:camel >] {

                type Inverse = [< Rename $asset:camel >];

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
