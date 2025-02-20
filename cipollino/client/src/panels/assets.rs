
use std::cell::RefCell;

use project::{alisa::{Action, AnyPtr, UnorderedChildList}, Asset, CreateFolder, Folder, FolderTreeData, Ptr, TransferFolder};

use crate::EditorState;

use super::Panel;

#[derive(Clone)]
struct AssetSelection {
    folders: Vec<Ptr<Folder>>
}

impl AssetSelection {

    fn transfer(self, new_parent: Ptr<Folder>, state: &EditorState) {
        let mut action = Action::new(); 
        for moved_folder in self.folders {
            state.client.perform(&mut action, TransferFolder {
                ptr: moved_folder,
                new_parent: new_parent 
            });
        }
        state.undo_redo.add(action);
    }

}

#[derive(Default)]
pub struct AssetsPanel {
    renaming_state: RefCell<Option<(AnyPtr, String)>>,
    started_renaming: RefCell<bool>,
    asset_dnd_source: RefCell<pierro::DndSource>,
}

impl AssetsPanel {

    fn render_folder_contents(&self,
        ui: &mut pierro::UI,
        folders: &UnorderedChildList<Folder>,
        state: &EditorState
    ) {
        for folder in folders.iter() {
            self.render_folder(ui, folder, state);
        }
    }

    fn renamable_asset_label<A: Asset>(&self, ui: &mut pierro::UI, curr_name: &String, ptr: Ptr<A>, state: &EditorState) {
        let mut renaming = self.renaming_state.borrow_mut();
        let renaming_state = &mut *renaming;

        let mut renaming = false;
        if let Some((curr_renaming, new_name)) = renaming_state {
            if *curr_renaming == ptr.any() {
                renaming = true;
                let text_edit = pierro::text_edit(ui, new_name);
                if *self.started_renaming.borrow() {
                    *self.started_renaming.borrow_mut() = false;
                    text_edit.response.request_focus(ui);
                }
                if text_edit.done_editing {
                    let mut action = Action::new();
                    A::rename(&state.client, &mut action, ptr, new_name.clone());
                    state.undo_redo.add(action);
                    *renaming_state = None;
                }
            }
        }
        if !renaming {
            pierro::label(ui, curr_name);
        }
    }

    fn start_rename<A: Asset>(&self, curr_name: &String, ptr: Ptr<A>) {
        *self.renaming_state.borrow_mut() = Some((ptr.any(), curr_name.clone()));
        *self.started_renaming.borrow_mut() = true;
    }

    fn asset_label_context_menu<A: Asset>(&self, ui: &mut pierro::UI, state: &EditorState, ptr: Ptr<A>, name: &String, response: &pierro::Response) {
        pierro::context_menu(ui, response, |ui| {
            if pierro::menu_button(ui, "Rename").mouse_clicked() {
                self.start_rename(name, ptr);
                pierro::close_context_menu(ui, response.id);
            }
            if pierro::menu_button(ui, "Delete").mouse_clicked() {
                let mut action = Action::new();
                <A as Asset>::delete(&state.client, &mut action, ptr);
                state.undo_redo.add(action);
                pierro::close_context_menu(ui, response.id);
            }
        });
    }

    fn render_folder(&self, ui: &mut pierro::UI, folder_ptr: Ptr<Folder>, state: &EditorState) {
        let Some(folder) = state.client.get(folder_ptr) else { return; };

        ui.push_id_seed(&folder_ptr);
        let (_, moved_assets) = pierro::dnd_drop_zone::<AssetSelection, _>(ui, |ui| {
            let folder_response = pierro::collapsing_header(ui, |ui| {
                self.renamable_asset_label(ui, &folder.name, folder_ptr, state);
            }, |ui| {
                self.render_folder_contents(ui, &folder.folders, state); 
            });
            self.asset_label_context_menu(ui, state, folder_ptr, &folder.name, &folder_response); 

            self.asset_dnd_source.borrow_mut().source(ui, &folder_response, AssetSelection {
                folders: vec![folder_ptr],
            });
        });

        if let Some(moved_assets) = moved_assets {
            moved_assets.transfer(folder_ptr, state);
        }
    }

}

impl Panel for AssetsPanel {

    fn title(&self) -> String {
        "Assets".to_owned()
    }

    fn render(&mut self, ui: &mut pierro::UI, state: &mut EditorState) {

        pierro::menu_bar(ui, |ui| {
            if pierro::icon_button(ui, pierro::icons::FOLDER).mouse_clicked() {
                if let Some(ptr) = state.client.next_ptr() {
                    let mut action = Action::new();
                    state.client.perform(&mut action, CreateFolder {
                        ptr,
                        parent: Ptr::null(),
                        data: FolderTreeData {
                            name: "Folder".to_owned(),
                            ..Default::default()
                        },
                    });
                    state.undo_redo.add(action);
                }
            }
        });

        let (_, moved_assets) = pierro::dnd_drop_zone_with_size::<AssetSelection, _>(ui, pierro::Size::fr(1.0), pierro::Size::fr(1.0), |ui| {
            pierro::scroll_area(ui, |ui| {
                self.render_folder_contents(ui, &state.client.folders, state); 
            });
        });
        if let Some(moved_assets) = moved_assets {
            moved_assets.transfer(Ptr::null(), state);
        }

        self.asset_dnd_source.borrow_mut().display(ui, |ui| {
            let Some(assets) = ui.memory().get_dnd_payload::<AssetSelection>() else {
                ui.memory().clear_dnd_payload();
                return;
            };
            let assets = assets.clone();
            for folder_ptr in assets.folders {
                let Some(folder) = state.client.get(folder_ptr) else { continue; };
                pierro::horizontal_fit_centered(ui, |ui| {
                    pierro::icon(ui, pierro::icons::FOLDER);
                    pierro::label(ui, &folder.name)
                });
            }
        });
    }

}
