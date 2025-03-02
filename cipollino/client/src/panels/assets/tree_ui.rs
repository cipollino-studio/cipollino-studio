
use project::{alisa::UnorderedChildList, Action, Asset, Clip, Folder, Ptr};

use crate::{EditorState, ProjectState};

use super::{AssetSelection, AssetUI, AssetsPanel};

impl AssetsPanel {

    fn renamable_asset_label<A: Asset>(&self, ui: &mut pierro::UI, curr_name: &String, ptr: Ptr<A>, state: &ProjectState) {
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
                    A::rename(&mut action, ptr, new_name.clone());
                    state.client.queue_action(action);
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

    fn asset_label_context_menu<A: AssetUI>(&self, ui: &mut pierro::UI, state: &ProjectState, ptr: Ptr<A>, name: &String, response: &pierro::Response) {
        pierro::context_menu(ui, response, |ui| {
            if pierro::menu_button(ui, "Rename").mouse_clicked() {
                self.start_rename(name, ptr);
                pierro::close_context_menu(ui, response.id);
            }
            if pierro::menu_button(ui, "Delete").mouse_clicked() {
                let selection = AssetSelection::single(ptr);
                state.delete_assets(selection);
                pierro::close_context_menu(ui, response.id);
            }
        });
    }

    fn render_asset<A: AssetUI>(&self, ui: &mut pierro::UI, asset_ptr: Ptr<A>, project: &ProjectState, editor: &mut EditorState) {
        if let Some(asset) = project.client.get(asset_ptr) {
            let (response, _) = pierro::horizontal_fit_centered(ui, |ui| {
                pierro::icon(ui, A::ICON);
                pierro::h_spacing(ui, 3.0);
                self.renamable_asset_label(ui, asset.name(), asset_ptr, project);
            });

            self.asset_dnd_source.borrow_mut().source_without_cursor_icon(ui, &response, || AssetSelection::single(asset_ptr));
            
            self.asset_label_context_menu(ui, project, asset_ptr, asset.name(), &response);

            if response.mouse_double_clicked() {
                A::on_open(asset_ptr, project, editor);
            }
        }
    }

    fn render_folder(&self, ui: &mut pierro::UI, folder_ptr: Ptr<Folder>, project: &ProjectState, editor: &mut EditorState) {
        let Some(folder) = project.client.get(folder_ptr) else { return; };

        ui.push_id_seed(&folder_ptr);
        let (_, moved_assets) = pierro::dnd_drop_zone::<AssetSelection, _>(ui, |ui| {
            let folder_response = pierro::collapsing_header(ui, |ui| {
                self.renamable_asset_label(ui, &folder.name, folder_ptr, project);
            }, |ui| {
                self.render_folder_contents(ui, &folder.folders, &folder.clips, project, editor); 
            });
            self.asset_label_context_menu(ui, project, folder_ptr, &folder.name, &folder_response); 

            self.asset_dnd_source.borrow_mut().source_without_cursor_icon(ui, &folder_response, || AssetSelection::single(folder_ptr));
        });

        if let Some(moved_assets) = moved_assets {
            moved_assets.transfer(folder_ptr, project);
        }
    }

    pub(crate) fn render_folder_contents(&self,
        ui: &mut pierro::UI,
        folders: &UnorderedChildList<Folder>,
        clips: &UnorderedChildList<Clip>,
        project: &ProjectState,
        editor: &mut EditorState
    ) {
        for folder in folders.iter() {
            self.render_folder(ui, folder, project, editor);
        }
        for clip in clips.iter() {
            self.render_asset(ui, clip, project, editor);
        } 
    }

}
