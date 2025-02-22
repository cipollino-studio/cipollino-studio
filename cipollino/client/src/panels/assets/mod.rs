
use std::{cell::RefCell, collections::HashSet};

use project::{alisa::AnyPtr, Action, Asset, Client, Clip, ClipTreeData, CreateClip, CreateFolder, Folder, FolderTreeData, Ptr};
use selection::AssetSelection;

use crate::{EditorState, State};

use super::Panel;

mod tree_ui;
mod menu_bar;
mod selection;

trait AssetUI: Asset {

    const ICON: &'static str;

    fn create(ptr: Ptr<Self>, parent: Ptr<Folder>, client: &Client, action: &mut Action);
    fn selection_list(selection: &AssetSelection) -> &HashSet<Ptr<Self>>;
    fn selection_list_mut(selection: &mut AssetSelection) -> &mut HashSet<Ptr<Self>>;

    /// Called when the asset is double-clicked in the UI
    fn on_open(_ptr: Ptr<Self>, _state: &mut EditorState) {

    }

}

impl AssetUI for Folder {
    const ICON: &'static str = pierro::icons::FOLDER;

    fn create(ptr: Ptr<Self>, parent: Ptr<Folder>, client: &Client, action: &mut Action) {
        client.perform(action, CreateFolder {
            ptr,
            parent,
            data: FolderTreeData::default(),
        });
    }

    fn selection_list(selection: &AssetSelection) -> &HashSet<Ptr<Self>> {
        &selection.folders
    }

    fn selection_list_mut(selection: &mut AssetSelection) -> &mut HashSet<Ptr<Self>> {
        &mut selection.folders
    }
}

impl AssetUI for Clip {
    const ICON: &'static str = pierro::icons::FILM_STRIP;

    fn create(ptr: Ptr<Self>, parent: Ptr<Folder>, client: &Client, action: &mut Action) {
        client.perform(action, CreateClip {
            ptr,
            parent,
            data: ClipTreeData::default(),
        });
    }

    fn selection_list(selection: &AssetSelection) -> &HashSet<Ptr<Self>> {
        &selection.clips
    }

    fn selection_list_mut(selection: &mut AssetSelection) -> &mut HashSet<Ptr<Self>> {
        &mut selection.clips
    }

    fn on_open(clip: Ptr<Self>, state: &mut EditorState) {
        state.open_clip = clip;
    }
}

#[derive(Default)]
pub struct AssetsPanel {
    renaming_state: RefCell<Option<(AnyPtr, String)>>,
    started_renaming: RefCell<bool>,
    asset_dnd_source: RefCell<pierro::DndSource>,
}

impl Panel for AssetsPanel {

    fn title(&self) -> String {
        "Assets".to_owned()
    }

    fn render(&mut self, ui: &mut pierro::UI, state: &mut State) {
        self.menu_bar(ui, &state.project);

        let (_, moved_assets) = pierro::dnd_drop_zone_with_size::<AssetSelection, _>(ui, pierro::Size::fr(1.0), pierro::Size::fr(1.0), |ui| {
            pierro::scroll_area(ui, |ui| {
                self.render_folder_contents(ui, &state.project.client.folders, &state.project.client.clips, &state.project, &mut state.editor); 
            });
        });
        if let Some(moved_assets) = moved_assets {
            moved_assets.transfer(Ptr::null(), &state.project);
        }

        self.asset_dnd_source.borrow_mut().display(ui, |ui| {
            let Some(assets) = ui.memory().get_dnd_payload::<AssetSelection>() else {
                ui.memory().clear_dnd_payload();
                return;
            };
            let assets = assets.clone();
            assets.render_contents(ui, &state.project.client); 
        });
    }

}
