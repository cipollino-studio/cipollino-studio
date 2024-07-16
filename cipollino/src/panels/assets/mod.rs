
use cipollino_project::{crdt::fractional_index::FractionalIndex, project::{action::Action, clip::Clip, folder::Folder, obj::{ObjPtr, ObjRef}}};

use crate::{app::AppSystems, editor::EditorState, util::ui::dnd::{dnd_drop_zone, draggable_label, draggable_widget}};

use super::Panel;

#[derive(Clone, Copy, PartialEq, Eq)]
enum AssetPtr {
    Folder(ObjPtr<Folder>),
    Clip(ObjPtr<Clip>)
}

#[derive(Default)]
pub struct Assets {
    editing_name: Option<(AssetPtr, String)>
}

enum AssetCommand {
    Rename(AssetPtr, String),
    Transfer(AssetPtr, ObjPtr<Folder>),
    OpenClip(ObjPtr<Clip>)
}

impl Panel for Assets {

    fn ui(&mut self, ui: &mut egui::Ui, state: &mut EditorState, _systems: &mut AppSystems) {
        egui::TopBottomPanel::top(ui.next_auto_id()).show_inside(ui, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button(egui_phosphor::regular::FOLDER).clicked() {
                    let root_folder_ptr = state.project.root_folder().ptr();
                    let mut action = Action::new();
                    state.client.add_folder(&mut state.project, root_folder_ptr, FractionalIndex::half(), "Folder".to_owned(), &mut action);
                    state.actions.push_action(action);
                }
                if ui.button(egui_phosphor::regular::FILM_STRIP).clicked() {
                    let root_folder_ptr = state.project.root_folder().ptr();
                    let mut action = Action::new();
                    state.client.add_clip(&mut state.project, root_folder_ptr, FractionalIndex::half(), "Clip".to_owned(), 100, &mut action);
                    state.actions.push_action(action);
                }
            });
        });

        let mut commands = Vec::new();
        egui::ScrollArea::both().show(ui, |ui| {
            self.render_folder_contents(ui, state, &state.project.root_folder(), &mut commands); 
            let (_, root_payload) = dnd_drop_zone::<AssetPtr, ()>(ui, |ui| {
                let available_size = ui.available_size();
                ui.allocate_exact_size(available_size.max(egui::vec2(available_size.x, 30.0)), egui::Sense::hover());
            });
            if let Some(root_payload) = root_payload {
                let asset = root_payload.as_ref().clone();
                commands.push(AssetCommand::Transfer(asset, state.project.root_folder().ptr()))
            }
        });

        for command in commands {
            if let AssetCommand::OpenClip(clip) = command {
                state.open_clip = clip;
                continue;
            } 

            let mut action = Action::new();
            match command {
                AssetCommand::Rename(AssetPtr::Folder(folder), new_name) => state.client.set_folder_name(&mut state.project, folder, new_name, &mut action),
                AssetCommand::Rename(AssetPtr::Clip(clip), new_name) => state.client.set_clip_name(&mut state.project, clip, new_name, &mut action),
                AssetCommand::Transfer(AssetPtr::Folder(folder), new_parent) => state.client.transfer_folder(&mut state.project, folder, new_parent, FractionalIndex::half(), &mut action),
                AssetCommand::Transfer(AssetPtr::Clip(clip), new_parent) => state.client.transfer_clip(&mut state.project, clip, new_parent, FractionalIndex::half(), &mut action),
                _ => {None}
            };
            state.actions.push_action(action);
        }
    }

}

impl Assets {

    fn render_folder_contents(&mut self, ui: &mut egui::Ui, state: &EditorState, folder: &ObjRef<Folder>, commands: &mut Vec<AssetCommand>) -> Option<bool> {
        let mut inner_hovered = false;

        for subfolder in folder.folders.iter_ref(&state.project.folders) {
            ui.push_id(subfolder.ptr(), |ui| {
                inner_hovered |= self.render_subfolder(ui, state, &subfolder, &subfolder.name, commands).unwrap_or(false);
            });
        }
        for clip in folder.clips.iter_ref(&state.project.clips) {
            let mut editing_name = false;
            if let Some((asset_ptr, name)) = &mut self.editing_name {
                if *asset_ptr == AssetPtr::Clip(clip.ptr()) {
                    editing_name = true;
                    let resp = ui.text_edit_singleline(name); 
                    if resp.lost_focus() {
                        commands.push(AssetCommand::Rename(AssetPtr::Clip(clip.ptr()), name.clone()));
                        self.editing_name = None;
                    }
                }
            }
            if !editing_name {
                let resp = draggable_label(ui, format!("{} {}", egui_phosphor::regular::FILM_STRIP, clip.name.value()).as_str(), AssetPtr::Clip(clip.ptr()));
                if resp.double_clicked() {
                    commands.push(AssetCommand::OpenClip(clip.ptr()));
                }
                resp.context_menu(|ui| {
                    if ui.button("Rename").clicked() {
                        self.editing_name = Some((AssetPtr::Clip(clip.ptr()), clip.name.clone()));
                        ui.close_menu();
                    }
                });
            }
        }

        Some(inner_hovered)
    }

    fn render_subfolder(&mut self, ui: &mut egui::Ui, state: &EditorState, folder: &ObjRef<Folder>, folder_name: &str, commands: &mut Vec<AssetCommand>) -> Option<bool> {

        if let Some((asset_ptr, name)) = &mut self.editing_name {
            if *asset_ptr == AssetPtr::Folder(folder.ptr()) {
                let resp = ui.text_edit_singleline(name);
                if resp.lost_focus() {
                    commands.push(AssetCommand::Rename(AssetPtr::Folder(folder.ptr()), name.clone()));
                    self.editing_name = None;
                }
                return Some(false);
            }
        }

        let mut frame = egui::Frame::default().begin(ui);
        let mut inner_hovered = false;
        let folder_resp = draggable_widget(&mut frame.content_ui, AssetPtr::Folder(folder.ptr()), |ui, _| {
            let resp = ui.collapsing(folder_name, |ui| {
                inner_hovered |= self.render_folder_contents(ui, state, folder, commands).unwrap_or(false); 
            }).header_response;
            (resp.clone(), resp)
        });

        folder_resp.context_menu(|ui| {
            if ui.button("Rename").clicked() {
                self.editing_name = Some((AssetPtr::Folder(folder.ptr()), folder.name.value().clone()));
                ui.close_menu();
            }
        });

        let response = frame.allocate_space(ui);

        let is_anything_being_dragged = egui::DragAndDrop::has_any_payload(ui.ctx());
        let can_accept_what_is_being_dragged = egui::DragAndDrop::has_payload_of_type::<AssetPtr>(ui.ctx());

        let (stroke, hovered) = if is_anything_being_dragged
            && can_accept_what_is_being_dragged
            && response.contains_pointer()
            && !inner_hovered {
            (ui.visuals().widgets.active.bg_stroke, true)
        } else {
            (ui.visuals().widgets.inactive.bg_stroke, false)
        };

        frame.frame.fill = egui::Color32::TRANSPARENT;
        frame.frame.stroke = stroke;

        frame.paint(ui);

        if !inner_hovered {
            if let Some(payload) = response.dnd_release_payload::<AssetPtr>() {
                let asset_ptr = payload.as_ref().clone();
                commands.push(AssetCommand::Transfer(asset_ptr, folder.ptr()));
            }
        }

        Some(hovered || inner_hovered)
    }

}
