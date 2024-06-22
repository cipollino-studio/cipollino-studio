
use cipollino_project::project::{action::Action, folder::Folder, obj::{ObjList, ObjPtr, ObjRef}};

use crate::app::{editor::EditorState, AppSystems};

use super::Panel;

#[derive(Default)]
pub struct Assets {

}

enum AssetCommand {
    RenameFolder(ObjPtr<Folder>, String) 
}

impl Panel for Assets {

    fn ui(&mut self, ui: &mut egui::Ui, state: &mut EditorState, systems: &mut AppSystems) {
        egui::TopBottomPanel::top(ui.next_auto_id()).show_inside(ui, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button(egui_phosphor::regular::FOLDER).clicked() {
                    let root_folder_ptr = state.project.root_folder().ptr();
                    state.client.add_folder(&mut state.project, root_folder_ptr, "Folder".to_owned());
                }
            });
        });

        let mut commands = Vec::new();

        ui.label(format!("{}", state.project.fps));
        self.folder_contents(ui, state.project.root_folder(), &state.project.folders, &mut commands);

        for command in commands {
            let mut action = Action::new();
            match command {
                AssetCommand::RenameFolder(folder, name) => state.client.set_folder_name(&mut state.project, folder, name, &mut action),
            };
            state.actions.push_action(action);
        }
    }

}

impl Assets {

    fn folder_contents(&self, ui: &mut egui::Ui, folder: ObjRef<Folder>, folder_list: &ObjList<Folder>, commands: &mut Vec<AssetCommand>) {
        ui.push_id(folder.ptr(), |ui| {
            if ui.collapsing(&*folder.name, |ui| {
                for child_folder in folder.folders.iter_ref(folder_list) {
                    self.folder_contents(ui, child_folder, folder_list, commands);
                }
            }).header_response.clicked() {
                commands.push(AssetCommand::RenameFolder(folder.ptr(), "Test".to_owned()))
            }
        });
    }

}
