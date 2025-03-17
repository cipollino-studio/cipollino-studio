
#[macro_export]
macro_rules! asset_transfer_operation {
    ($asset: ty) => {
        alisa::paste::paste! {

            #[derive(alisa::Serializable)]
            #[project(crate::Project)]
            pub struct [< Transfer $asset:camel >] {
                pub ptr: alisa::Ptr<$asset>,
                pub new_folder: alisa::Ptr<crate::Folder>
            }

            impl Default for [< Transfer $asset:camel >] {

                fn default() -> Self {
                    Self {
                        ptr: alisa::Ptr::null(),
                        new_folder: alisa::Ptr::null()
                    }
                }

            }

            impl alisa::Operation for [< Transfer $asset:camel >] {

                type Project = crate::Project;

                const NAME: &'static str = stringify!([< Transfer $asset:camel >]);

                fn perform(&self, recorder: &mut alisa::Recorder<Self::Project>) -> bool {
                    use alisa::TreeObj;
                    if alisa::transfer_tree_object(recorder, self.ptr, &self.new_folder, &()) {
                        // Fix the name of the folder
                        let context = recorder.context();
                        let Some(child_list) = $asset::child_list(self.new_folder, &context) else { return false; };
                        let sibling_names = $asset::get_sibling_names(child_list, recorder.obj_list(), Some(self.ptr));
                        crate::rectify_name_duplication(self.ptr, sibling_names, recorder);

                        true
                    } else {
                        false
                    }
                }

            }

            impl alisa::InvertibleOperation for [< Transfer $asset:camel >] {

                type Inverse = [< Transfer $asset:camel >];

                fn inverse(&self, context: &alisa::ProjectContext<Self::Project>) -> Option<Self::Inverse> {
                    use alisa::TreeObj;
                    let asset = context.obj_list().get(self.ptr)?;
                    Some(Self {
                        ptr: self.ptr,
                        new_folder: asset.parent()
                    })
                }

            }

        }
    };
}
