
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::path::PathBuf;

use project::{alisa::rmpv, Ptr};
use project::{deep_load_clip, Client};

use crate::{AppSystems, AssetList, DockingLayoutPref, EditorPanel, PencilTool};

mod socket;
pub use socket::*;

mod state;
pub use state::*;

mod selection;
pub use selection::*;

mod shortcuts;

pub struct ProjectState {
    pub client: project::Client,

    assets_to_delete: RefCell<Vec<AssetList>>
}

impl ProjectState {

    pub fn delete_assets(&self, selection: AssetList) {
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
                    
                    curr_tool: Rc::new(RefCell::new(Box::new(PencilTool::default()))),

                    selection: Selection::new(),

                    stroke_mesh_cache: HashMap::new(),
                    stroke_preview: None,

                    color: pierro::Color::BLACK
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

        // Menu bar
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

        // Collab
        if let Some(socket) = &mut self.socket {
            for to_send in self.state.project.client.take_messages() {
                socket.send(to_send);
            }
            while let Some(msg) = socket.receive() {
                self.state.project.client.receive_message(msg, &mut ());
            }
            
            if !socket.has_signal() {
                socket.set_signal(ui.redraw_signal());
            }
        }

        self.state.editor.selection.begin_frame(ui.key_down(&pierro::Key::SHIFT));

        // Render the docking panels 
        if self.docking.render(ui, &mut self.state) {
            // Save the layout if it was modified
            systems.prefs.set::<DockingLayoutPref>(&self.docking);
        }

        self.state.editor.selection.end_frame(ui.input().l_mouse.clicked() || ui.input().l_mouse.drag_started());

        // Load the currently open clip if it's not open
        if let Some(clip) = self.state.project.client.get(self.state.editor.open_clip) {
            if let Some(clip_inner) = self.state.project.client.get(clip.inner) {
                self.state.editor.tick_playback(ui, clip_inner);
            } else {
                deep_load_clip(self.state.editor.open_clip, &self.state.project.client);
            }
        }

        self.use_shortcuts(ui, systems);

        // Update the project client
        self.state.project.tick();
        self.state.project.client.tick(&mut ());
    }

}
