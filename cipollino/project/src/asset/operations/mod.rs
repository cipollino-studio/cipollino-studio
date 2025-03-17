
use std::collections::HashSet;

use crate::Project;

use super::Asset;

mod creation;
mod rename;
mod transfer;

fn find_valid_name<A: Asset>(mut name: &str, sibling_names: &HashSet<String>) -> String {
    if name.is_empty() {
        name = A::NAME;
    } 
    if sibling_names.contains(name) {
        let mut potential_names = (1..).map(|idx| format!("{} ({})", name, idx));
        potential_names.find(|name| !sibling_names.contains(name.as_str())).unwrap()
    } else {
        name.to_owned()
    }
}

pub(crate) fn rectify_name_duplication<A: Asset>(ptr: alisa::Ptr<A>, sibling_names: HashSet<String>, recorder: &mut alisa::Recorder<Project>) {
    let Some(asset) = recorder.get_obj_mut(ptr) else { return; };
    let asset_name = asset.name().as_str(); 
    if sibling_names.contains(asset_name) || asset_name.is_empty() {
        let new_name = find_valid_name::<A>(asset_name, &sibling_names);
        *asset.name_mut() = new_name;
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