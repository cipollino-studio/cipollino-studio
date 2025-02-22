
use std::collections::HashSet;

use crate::Project;

use super::Asset;

mod creation;
mod rename;
mod transfer;

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

        crate::asset_creation_operations!($asset);
        crate::asset_rename_operation!($asset);
        crate::asset_transfer_operation!($asset);

    };
}