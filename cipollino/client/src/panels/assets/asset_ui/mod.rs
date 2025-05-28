
use std::collections::HashSet;

use project::{Action, Asset, Client, Folder, Ptr};

use crate::{EditorState, ProjectState};

use super::AssetList;

mod folder;
mod clip;
mod palette;

pub trait AssetUI: Asset {

    const ICON: &'static str;

    fn create(client: &Client, ptr: Ptr<Self>, parent: Ptr<Folder>, action: &mut Action);
    fn asset_list(list: &AssetList) -> &HashSet<Ptr<Self>>;
    fn asset_list_mut(list: &mut AssetList) -> &mut HashSet<Ptr<Self>>;
    fn context_menu(_ui: &mut pierro::UI, _project: &ProjectState, _editor: &mut EditorState, _ptr: Ptr<Self>, _context_menu_id: pierro::Id) {
        
    }

    /// Called when the asset is double-clicked in the UI
    fn on_open(_ptr: Ptr<Self>, _project: &ProjectState, _state: &mut EditorState) {

    }

    fn label_ui(_ui: &mut pierro::UI, _ptr: Ptr<Self>, _project: &ProjectState, _state: &mut EditorState) {

    }

}
