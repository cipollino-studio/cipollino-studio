use std::collections::HashSet;

use alisa::Ptr;
use project::{Action, Client, CreateFolder, Folder, FolderTreeData};

use crate::AssetList;

use super::AssetUI;


impl AssetUI for Folder {
    const ICON: &'static str = pierro::icons::FOLDER;

    fn create(_client: &Client, ptr: Ptr<Self>, parent: Ptr<Folder>, action: &mut Action) {
        action.push(CreateFolder {
            ptr,
            parent,
            data: FolderTreeData::default(),
        });
    }

    fn asset_list(list: &AssetList) -> &HashSet<Ptr<Self>> {
        &list.folders
    }

    fn asset_list_mut(list: &mut AssetList) -> &mut HashSet<Ptr<Self>> {
        &mut list.folders
    }

}
