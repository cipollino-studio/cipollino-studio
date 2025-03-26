
use std::{cell::RefCell, collections::HashSet};

use project::{alisa::AnyPtr, Action, Asset, Clip, ClipTreeData, CreateClip, CreateFolder, Folder, FolderTreeData, Ptr};

use crate::{EditorState, ProjectState};

use super::{Panel, PanelContext};

mod tree_ui;
mod menu_bar;

mod list;
pub use list::*;

mod clip_dialog;
pub use clip_dialog::*;
pub trait AssetUI: Asset {

    const ICON: &'static str;

    fn create(ptr: Ptr<Self>, parent: Ptr<Folder>, action: &mut Action);
    fn asset_list(list: &AssetList) -> &HashSet<Ptr<Self>>;
    fn asset_list_mut(list: &mut AssetList) -> &mut HashSet<Ptr<Self>>;
    fn context_menu(_ui: &mut pierro::UI, _project: &ProjectState, _editor: &mut EditorState, _ptr: Ptr<Self>, _context_menu_id: pierro::Id) {
        
    }

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

    fn context_menu(ui: &mut pierro::UI, project: &ProjectState, editor: &mut EditorState, clip_ptr: Ptr<Self>, context_menu_id: pierro::Id) {
        if pierro::menu_button(ui, "Properties...").mouse_clicked() {
            if let Some(clip) = project.client.get(clip_ptr) {
                let name = clip.name.clone();
                let clip_inner_ptr = clip.inner;
                editor.on_load(project, clip_inner_ptr, move |_, editor, clip_inner| {
                    let properties = ClipProperties {
                        name: name.clone(),
                        length: clip_inner.length,
                        framerate: clip_inner.framerate,
                        width: clip_inner.width,
                        height: clip_inner.height,
                    };

                    editor.open_window(ClipPropertiesDialog {
                        properties,
                        clip_ptr,
                        clip_inner_ptr
                    });
                });
            }
            pierro::close_context_menu(ui, context_menu_id);
        }
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

    fn render(&mut self, ui: &mut pierro::UI, context: &mut PanelContext) {
        self.menu_bar(ui, context.editor, context.project);

        let (_, moved_assets) = pierro::dnd_drop_zone_with_size::<AssetList, _>(ui, pierro::Size::fr(1.0), pierro::Size::fr(1.0), |ui| {
            pierro::scroll_area(ui, |ui| {
                pierro::margin(ui, pierro::Margin::same(3.0), |ui| {
                    self.render_folder_contents(ui, &context.project.client.folders, &context.project.client.clips, &context.project, &mut context.editor); 
                });
            });
        });
        if let Some(moved_assets) = moved_assets {
            moved_assets.transfer(Ptr::null(), &context.project, &context.editor);
        }

        self.asset_dnd_source.borrow_mut().display(ui, |ui| {
            let Some(assets) = ui.memory().get_dnd_payload::<AssetList>() else {
                ui.memory().clear_dnd_payload();
                return;
            };
            let assets = assets.clone();
            assets.render_contents(ui, &context.project.client); 
        });
    }

}
