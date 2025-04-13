
use project::{alisa::UnorderedChildList, Action, ActionContext, Asset, Clip, Folder, Ptr};

use crate::{EditorState, ProjectState};

use super::{AssetList, AssetUI, AssetsPanel};

impl AssetsPanel {

    fn renamable_asset_label<A: Asset>(&self, ui: &mut pierro::UI, curr_name: &String, ptr: Ptr<A>, project: &ProjectState, open_clip: Ptr<Clip>, time: f32) -> Option<pierro::Response> {
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
                    let mut action = Action::new(ActionContext::new(format!("Rename {}", A::NAME), open_clip, time));
                    A::rename(&mut action, ptr, new_name.clone());
                    project.client.queue_action(action);
                    *renaming_state = None;
                }
            }
        }

        if !renaming {
            Some(pierro::label(ui, curr_name))
        } else {
            None
        }
    }

    fn start_rename<A: Asset>(&self, curr_name: &String, ptr: Ptr<A>) {
        *self.renaming_state.borrow_mut() = Some((ptr.any(), curr_name.clone()));
        *self.started_renaming.borrow_mut() = true;
    }

    fn asset_label_context_menu<A: AssetUI>(&self, ui: &mut pierro::UI, project: &ProjectState, editor: &mut EditorState, ptr: Ptr<A>, name: &String, response: &pierro::Response) {
        pierro::context_menu(ui, response, |ui| {
            if pierro::menu_button(ui, "Rename").mouse_clicked() {
                self.start_rename(name, ptr);
                pierro::close_context_menu(ui, response.id);
            }
            if pierro::menu_button(ui, "Delete").mouse_clicked() {
                let selection = AssetList::single(ptr);
                project.delete_assets(selection);
                pierro::close_context_menu(ui, response.id);
            }
            A::context_menu(ui, project, editor, ptr, response.id);
        });
    }

    fn render_asset<A: AssetUI>(&self, ui: &mut pierro::UI, asset: &A, asset_ptr: Ptr<A>, project: &ProjectState, editor: &mut EditorState) {
        
        // Render the asset
        let (response, (label_resp, icon_resp)) = pierro::horizontal_fit_centered(ui, |ui| {
            let icon_resp = pierro::icon(ui, A::ICON);
            pierro::h_spacing(ui, 3.0);
            let label_resp = self.renamable_asset_label(ui, asset.name(), asset_ptr, project, editor.open_clip, editor.time);
            A::label_ui(ui, asset_ptr, editor);
            (label_resp, icon_resp)
        });

        // Hover/click animation
        let text_color = ui.style::<pierro::theme::TextColor>();
        if let Some(label_resp) = label_resp {
            pierro::button_text_color_animation(ui, label_resp.node_ref, &response, text_color);
        }
        pierro::button_text_color_animation(ui, icon_resp.node_ref, &response, text_color);

        self.asset_dnd_source.borrow_mut().source_without_cursor_icon(ui, &response, || AssetList::single(asset_ptr));
        
        self.asset_label_context_menu(ui, project, editor, asset_ptr, asset.name(), &response);

        // Opening
        if response.mouse_double_clicked() {
            A::on_open(asset_ptr, project, editor);
        }
    }

    fn render_assets<A: AssetUI>(&self, ui: &mut pierro::UI, project: &ProjectState, editor: &mut EditorState, assets: &UnorderedChildList<project::alisa::LoadingPtr<A>>) {
        let mut assets = assets.iter().filter_map(|ptr| {
            let asset = project.client.get(ptr.ptr())?;
            Some((asset.name(), asset, ptr.ptr()))
        }).collect::<Vec<_>>();
        assets.sort_by_key(|(name, _, _)| *name);
        for (_, asset, asset_ptr) in assets {
            self.render_asset(ui, asset, asset_ptr, project, editor);
        }
    }

    fn render_folder(&self, ui: &mut pierro::UI, folder: &Folder, folder_ptr: Ptr<Folder>, project: &ProjectState, editor: &mut EditorState) {
        let text_color = ui.style::<pierro::theme::TextColor>();

        ui.push_id_seed(&folder_ptr);
        let (_, moved_assets) = pierro::dnd_drop_zone::<AssetList, _>(ui, |ui| {
            let open_clip = editor.open_clip;
            let time = editor.time;
            let folder_response = pierro::collapsing_header(ui, |ui, response| {
                if let Some(folder_name_response) = self.renamable_asset_label(ui, &folder.name, folder_ptr, project, open_clip, time) {
                    pierro::button_text_color_animation(ui, folder_name_response.node_ref, response, text_color);
                }
            }, |ui| {
                self.render_folder_contents(ui, &folder.folders, &folder.clips, project, editor); 
            });
            self.asset_label_context_menu(ui, project, editor, folder_ptr, &folder.name, &folder_response); 

            self.asset_dnd_source.borrow_mut().source_without_cursor_icon(ui, &folder_response, || AssetList::single(folder_ptr));
        });

        if let Some(moved_assets) = moved_assets {
            moved_assets.transfer(folder_ptr, project, &editor);
        }
    }

    pub(crate) fn render_folder_contents(&self,
        ui: &mut pierro::UI,
        folders: &UnorderedChildList<project::alisa::LoadingPtr<Folder>>,
        clips: &UnorderedChildList<project::alisa::LoadingPtr<Clip>>,
        project: &ProjectState,
        editor: &mut EditorState
    ) {

        let mut folders = folders.iter().filter_map(|ptr| {
            let folder = project.client.get(ptr.ptr())?;
            Some((folder.name(), folder, ptr.ptr()))
        }).collect::<Vec<_>>();
        folders.sort_by_key(|(name, _, _)| *name);

        for (_, folder, ptr) in folders {
            self.render_folder(ui, folder, ptr, project, editor);
        }

        self.render_assets(ui, project, editor, clips);
    }

}
