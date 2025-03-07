
use std::cell::RefCell;
use std::path::PathBuf;

use project::{alisa::rmpv, Ptr};
use project::{deep_load_clip, Client};

use crate::{AppSystems, AssetSelection, DockingLayoutPref, EditorPanel, Pencil};

mod socket;
pub use socket::*;

mod state;
pub use state::*;

pub struct ProjectState {
    pub client: project::Client,

    assets_to_delete: RefCell<Vec<AssetSelection>>
}

impl ProjectState {

    pub fn delete_assets(&self, selection: AssetSelection) {
        self.assets_to_delete.borrow_mut().push(selection);
    }

    pub fn tick(&self) {
        let to_delete = self.assets_to_delete.borrow_mut().pop();
        if let Some(to_delete) = to_delete {
            if !to_delete.try_delete(&self.client) {
                to_delete.deep_load_all(&self.client);
                self.assets_to_delete.borrow_mut().push(to_delete);
            }
        }
    }

}

pub struct State {
    pub project: ProjectState,
    pub editor: EditorState,
    pub renderer: Option<malvina::Renderer>
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
                    assets_to_delete: RefCell::new(Vec::new())
                },
                editor: EditorState {
                    time: 0.0,
                    playing: false,

                    open_clip: Ptr::null(),
                    active_layer: Ptr::null(),
                    
                    curr_tool: Box::new(Pencil::default())
                },
                renderer: None
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
                    self.state.project.client.undo();
                }
                if pierro::menu_button(ui, "Redo").mouse_clicked() {
                    self.state.project.client.redo();
                }
            });
        });

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

        if let Some(clip) = self.state.project.client.get(self.state.editor.open_clip) {
            if let Some(clip_inner) = self.state.project.client.get(clip.inner) {
                self.state.editor.tick_playback(ui, clip_inner);
            } else {
                deep_load_clip(self.state.editor.open_clip, &self.state.project.client);
            }
        }

        self.state.project.tick();
        self.state.project.client.tick(&mut ());
    }

}
