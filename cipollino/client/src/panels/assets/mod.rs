
use std::{cell::RefCell, collections::HashSet};

use project::{alisa::AnyPtr, Action, Asset, Clip, ClipTreeData, CreateClip, CreateFolder, Folder, FolderTreeData, Ptr};

use crate::{EditorState, ProjectState, State};

use super::Panel;

mod tree_ui;
mod menu_bar;

mod list;
pub use list::*;

mod clip_dialog;

pub trait AssetUI: Asset {

    const ICON: &'static str;

    fn create(ptr: Ptr<Self>, parent: Ptr<Folder>, action: &mut Action);
    fn asset_list(list: &AssetList) -> &HashSet<Ptr<Self>>;
    fn asset_list_mut(list: &mut AssetList) -> &mut HashSet<Ptr<Self>>;

    /// Called when the asset is double-clicked in the UI
    fn on_open(_ptr: Ptr<Self>, _project: &ProjectState, _state: &mut EditorState) {

    }

}

impl AssetUI for Folder {
    const ICON: &'static str = pierro::icons::FOLDER;

    fn create(ptr: Ptr<Self>, parent: Ptr<Folder>, action: &mut Action) {
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

impl AssetUI for Clip {
    const ICON: &'static str = pierro::icons::FILM_STRIP;

    fn create(ptr: Ptr<Self>, parent: Ptr<Folder>, action: &mut Action) {
        action.push(CreateClip {
            ptr,
            parent,
            data: ClipTreeData::default(),
        });
    }

    fn on_open(clip: Ptr<Self>, _project: &ProjectState, state: &mut EditorState) {
        state.open_clip(clip);
    }

    fn asset_list(list: &AssetList) -> &HashSet<Ptr<Self>> {
        &list.clips
    }

    fn asset_list_mut(list: &mut AssetList) -> &mut HashSet<Ptr<Self>> {
        &mut list.clips
    }

}

#[derive(Default)]
pub struct AssetsPanel {
    renaming_state: RefCell<Option<(AnyPtr, String)>>,
    started_renaming: RefCell<bool>,
    asset_dnd_source: RefCell<pierro::DndSource>,
}

impl Panel for AssetsPanel {

    const NAME: &'static str = "Assets";

    fn title(&self) -> String {
        "Assets".to_owned()
    }

    fn render(&mut self, ui: &mut pierro::UI, state: &mut State) {
        self.menu_bar(ui, state);

        let (_, moved_assets) = pierro::dnd_drop_zone_with_size::<AssetList, _>(ui, pierro::Size::fr(1.0), pierro::Size::fr(1.0), |ui| {
            pierro::scroll_area(ui, |ui| {
                self.render_folder_contents(ui, &state.project.client.folders, &state.project.client.clips, &state.project, &mut state.editor); 
            });
        });
        if let Some(moved_assets) = moved_assets {
            moved_assets.transfer(Ptr::null(), &state.project);
        }

        self.asset_dnd_source.borrow_mut().display(ui, |ui| {
            let Some(assets) = ui.memory().get_dnd_payload::<AssetList>() else {
                ui.memory().clear_dnd_payload();
                return;
            };
            let assets = assets.clone();
            assets.render_contents(ui, &state.project.client); 
        });
    }

}
