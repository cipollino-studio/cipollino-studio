
use alisa::Children;

use crate::project::{folder::Folder, Project};

pub struct Context {
    pub server: alisa::Server<Project>
}

pub struct ClientTab {
    pub client_id: alisa::ClientId,
    pub client: alisa::Client<Project>,
    pub outgoing_msgs: Vec<rmpv::Value> 
}

impl ClientTab {

    fn render_folder(&self, ui: &mut pierro::UI, folder_ptr: alisa::Ptr<Folder>) {
        let Some(folder) = self.client.get(folder_ptr) else { return; };
        let (_, _) = pierro::dnd_source(ui, folder_ptr, |ui| {
            pierro::collapsing_header(ui, &folder.name, |ui| {
                if pierro::button(ui, "Rename").mouse_clicked() {
                    // self.client.perform(&mut action, SetFolderName {
                    //     ptr: folder_ptr,
                    //     name_value: folder.name.clone() + "!",
                    // });
                    // self.actions.add(action);
                }
                for subfolder in folder.folders.iter() {
                    self.render_folder(ui, subfolder);
                }
            });
        });
        // if let Some(moved_folder) = pierro::dnd_receive_payload::<alisa::Ptr<Folder>>(ui, &response) {
            // self.client.perform(&mut action, TransferFolder {
            //     ptr: moved_folder,
            //     new_parent: folder_ptr,
            //     new_idx: (),
            // });
            // self.actions.add(action);
        // }
    }

}

impl pierro::DockingTab for ClientTab {

    type Context = Context;

    fn title(&self) -> String {
        format!("Client #{:?}", self.client_id) 
    }

    fn render(&mut self, ui: &mut pierro::UI, context: &mut Context) {
        pierro::label(ui, format!("n: {}", self.client.project().n));

        if pierro::button(ui, "Set to 50").mouse_clicked() {
            // let mut action = alisa::Action::new();
            // self.client.perform(&mut action, SetN {
            //     n: 50,
            // });
            // self.actions.add(action);
        }

        pierro::horizontal(ui, |ui| { 
            if pierro::icon_button(ui, pierro::icons::PLUS).mouse_clicked() {
                // self.client.perform(&mut action, IncrN);
                // self.actions.add(action);
            }
            if pierro::icon_button(ui, pierro::icons::MINUS).mouse_clicked() {
                // let mut action = alisa::Action::new();
                // self.client.perform(&mut action, DecrN);
                // self.actions.add(action);
            }
        });

        pierro::v_spacing(ui, 10.0);

        pierro::label(ui, format!("Number of folders: {}", self.client.project().folders.n_children()));
        for folder_ptr in self.client.project().folders.iter() {
            self.render_folder(ui, folder_ptr); 
        }
        pierro::horizontal(ui, |ui| { 
            if pierro::icon_button(ui, pierro::icons::PLUS).mouse_clicked() {
                // if let Some(ptr) = self.client.next_ptr() {
                    // let mut action = alisa::Action::new();
                    // self.client.perform(&mut action, CreateFolder {
                    //     ptr,
                    //     parent: alisa::Ptr::null(),
                    //     idx: (),
                    //     data: FolderTreeData {
                    //         name: "Folder".to_string(),
                    //         folders: alisa::UnorderedChildListTreeData::default(),
                    //     } 
                    // });
                    // self.actions.add(action);
                // }
            }
            if pierro::icon_button(ui, pierro::icons::MINUS).mouse_clicked() {
                // let mut action = alisa::Action::new();
                // self.client.perform(&mut action, DecrN);
                // self.actions.add(action);
            }
        });

        pierro::v_spacing(ui, 10.0);

        pierro::horizontal(ui, |ui| {
            if pierro::button(ui, "<").mouse_clicked() {
                // self.actions.undo(&self.client);
            }
            if pierro::button(ui, ">").mouse_clicked() {
                // self.actions.redo(&self.client);
            }
        });

        self.client.tick(&mut ());
        self.outgoing_msgs.append(&mut self.client.take_messages());

        pierro::v_spacing(ui, 20.0);
        pierro::label(ui, format!("# Outgoing messages queued: {}", self.outgoing_msgs.len()));
        if self.outgoing_msgs.len() > 0 {
            pierro::label(ui, format!("Next to send: {}", self.outgoing_msgs[0].to_string()));
            if pierro::button(ui, "Send!").mouse_clicked() {
                let msg = self.outgoing_msgs.remove(0);
                context.server.receive_message(self.client_id, msg);
            }
        }

        pierro::v_spacing(ui, 20.0);
        if let Some(incoming_msgs) = context.server.get_msgs_to_send(self.client_id) {
            pierro::label(ui, format!("# Incoming messages queued: {}", incoming_msgs.len()));
            if incoming_msgs.len() > 0 {
                pierro::label(ui, format!("Next to receive: {}", incoming_msgs[0].to_string()));
                if pierro::button(ui, "Receive!").mouse_clicked() {
                    let msg = incoming_msgs.remove(0);
                    self.client.receive_message(msg, &mut ());
                }
            }
        }
    }

    fn add_tab_dropdown<F: FnMut(Self)>(ui: &mut pierro::UI, mut add_tab: F, context: &mut Context) {
        if pierro::menu_button(ui, "Add Client").mouse_clicked() {
            let (client_id, welcome_data) = context.server.add_client();
            if let Some(client) = alisa::Client::collab(&welcome_data) {
                add_tab(ClientTab {
                    client_id,
                    client,
                    outgoing_msgs: Vec::new()
                });
            }
        }
    }

}

pub struct App {
    pub context: Context,
    pub docking: pierro::DockingState<ClientTab>
}

impl pierro::App for App {

    fn tick(&mut self, ui: &mut pierro::UI) {
        pierro::menu_bar(ui, |ui| {
            pierro::menu_bar_item(ui, "Server State", |ui| {
                pierro::label(ui, format!("n: {}", self.context.server.project().n));
            });
        });
        self.docking.render(ui, &mut self.context);
    }

}
