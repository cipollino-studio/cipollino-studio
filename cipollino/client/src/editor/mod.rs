
use std::path::PathBuf;

use project::{alisa::rmpv, Ptr};
use project::{Client, Clip};

use crate::{AppSystems, DockingLayoutPref, EditorPanel};

mod socket;
pub use socket::*;

pub struct ProjectState {
    pub client: project::Client,
    pub undo_redo: project::UndoRedoManager
}

pub struct EditorState {
    pub open_clip: Ptr<Clip>
}

pub struct State {
    pub project: ProjectState,
    pub editor: EditorState
}

pub struct Editor {
    state: State,
    docking: pierro::DockingState<EditorPanel>,
    socket: Option<Socket>
}

impl Editor {

    fn new(client: Client, socket: Option<Socket>, systems: &mut AppSystems) -> Self {
        Self {
            state: State {
                project: ProjectState {
                    client,
                    undo_redo: project::UndoRedoManager::new(),
                },
                editor: EditorState {
                    open_clip: Ptr::null(),
                },
            },
            docking: systems.prefs.get::<DockingLayoutPref>(),
            socket
        }
    }

    pub fn local(path: PathBuf, systems: &mut AppSystems) -> Option<Self> {
        Some(Self::new(Client::local(path)?, None, systems))
    }

    pub fn collab(socket: Socket, welcome_msg: &rmpv::Value, systems: &mut AppSystems) -> Option<Self> {
        Some(Self::new(Client::collab(welcome_msg)?, Some(socket), systems))
    }

    pub fn tick(&mut self, ui: &mut pierro::UI, systems: &mut AppSystems) {

        pierro::menu_bar(ui, |ui| {
            pierro::menu_bar_item(ui, "File", |ui| {
                if pierro::menu_button(ui, "Open").mouse_clicked() {
                    
                }
            });
            pierro::menu_bar_item(ui, "Edit", |ui| {
                if pierro::menu_button(ui, "Undo").mouse_clicked() {
                    self.state.project.undo_redo.undo(&self.state.project.client);
                }
                if pierro::menu_button(ui, "Redo").mouse_clicked() {
                    self.state.project.undo_redo.redo(&self.state.project.client);
                }
            });
        });
        self.state.project.client.tick(&mut ());

        if let Some(socket) = &mut self.socket {
            for to_send in self.state.project.client.take_messages() {
                socket.send(to_send);
            }
            while let Some(msg) = socket.receive() {
                self.state.project.client.receive_message(msg, &mut ());
            }
        }

        if self.docking.render(ui, &mut self.state) {
            systems.prefs.set::<DockingLayoutPref>(&self.docking);
        }
    }

}
