use std::collections::HashSet;

use crate::{Action, Client, Folder, Project};

pub trait Asset: alisa::TreeObj<ParentPtr = alisa::Ptr<Folder>, Project = Project, ChildList = alisa::UnorderedChildList<Self>> {

    fn name(&self) -> &String;    
    fn name_mut(&mut self) -> &mut String;

    fn rename(client: &Client, action: &mut Action, ptr: alisa::Ptr<Self>, name: String);
    fn delete(client: &Client, action: &mut Action, ptr: alisa::Ptr<Self>);

    fn get_sibling_names(child_list: &Self::ChildList, objects: &alisa::ObjList<Self>, exclude: Option<alisa::Ptr<Self>>) -> HashSet<String> {
        child_list.iter()
            .filter(|ptr| Some(*ptr) != exclude)
            .filter_map(|ptr| objects.get(ptr)).map(|asset| asset.name().clone())
            .collect()
    }

}

pub(crate) struct SetAssetNameDelta<A: Asset> {
    pub ptr: alisa::Ptr<A>,
    pub name: String
}

impl<A: Asset> alisa::Delta for SetAssetNameDelta<A> {
    type Project = A::Project;

    fn perform(&self, context: &mut alisa::ProjectContextMut<'_, Self::Project>) {
        let Some(asset) = context.obj_list_mut().get_mut(self.ptr) else { return; };
        *asset.name_mut() = self.name.clone();
    }
}

pub(crate) fn rectify_name_duplication<A: Asset>(ptr: alisa::Ptr<A>, sibling_names: HashSet<String>, recorder: &mut alisa::Recorder<Project>) {
    let Some(asset) = recorder.obj_list_mut().get_mut(ptr) else { return; };
    let asset_name = asset.name().as_str(); 
    if sibling_names.contains(asset_name) {
        let old_name = asset_name.to_owned();
        let mut potential_names = (1..).map(|idx| format!("{} ({})", asset_name, idx));
        let new_name = potential_names.find(|name| !sibling_names.contains(name.as_str())).unwrap();
        *asset.name_mut() = new_name;
        recorder.push_delta(SetAssetNameDelta {
            ptr,
            name: old_name,
        });
    }
} 

#[macro_export]
macro_rules! asset_operations {
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
    };
}
