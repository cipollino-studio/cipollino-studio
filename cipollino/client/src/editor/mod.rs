
use std::path::PathBuf;

use project::alisa::rmpv;
use project::Client;

use crate::{AssetsPanel, EditorPanel, ScenePanel};

mod socket;
pub use socket::*;

pub struct EditorState {
    pub client: project::Client,
    pub undo_redo: project::UndoRedoManager
}

pub struct Editor {
    state: EditorState,
    docking: pierro::DockingState<EditorPanel>,
    socket: Option<Socket>
}

impl Editor {

    fn new(client: Client, socket: Option<Socket>) -> Self {
        Self {
            state: EditorState {
                client,
                undo_redo: project::UndoRedoManager::new(),
            },
            docking: pierro::DockingState::new(vec![
                EditorPanel::new::<ScenePanel>(),
                EditorPanel::new::<AssetsPanel>(),
            ]),
            socket
        }
    }

    pub fn local(path: PathBuf) -> Option<Self> {
        Some(Self::new(Client::local(path)?, None))
    }

    pub fn collab(socket: Socket, welcome_msg: &rmpv::Value) -> Option<Self> {
        Some(Self::new(Client::collab(welcome_msg)?, Some(socket)))
    }

    pub fn tick(&mut self, ui: &mut pierro::UI) {

        pierro::menu_bar(ui, |ui| {
            pierro::menu_bar_item(ui, "File", |ui| {
                if pierro::menu_button(ui, "Open").mouse_clicked() {
                    
                }
            });
            pierro::menu_bar_item(ui, "Edit", |ui| {
                if pierro::menu_button(ui, "Undo").mouse_clicked() {
                    self.state.undo_redo.undo(&self.state.client);
                }
                if pierro::menu_button(ui, "Redo").mouse_clicked() {
                    self.state.undo_redo.redo(&self.state.client);
                }
            });
        });
        self.state.client.tick(&mut ());

        if let Some(socket) = &mut self.socket {
            for to_send in self.state.client.take_messages() {
                socket.send(to_send);
            }
            while let Some(msg) = socket.receive() {
                self.state.client.receive_message(msg, &mut ());
            }
        }

        self.docking.render(ui, &mut self.state);
    }

}
