
use std::path::PathBuf;

use project::alisa::rmpv;
use project::{deep_load_clip, Client};

use crate::{AppSystems, DockingLayoutPref, EditorPanel};

mod socket;
pub use socket::*;

mod state;
pub use state::*;

mod selection;
pub use selection::*;

mod shortcuts;

mod menu_bar;

mod export;
use export::*;

pub struct Editor {
    state: State,
    docking: pierro::DockingState<EditorPanel>,
    windows: pierro::WindowManager<State>,
    socket: Option<Socket>
}

impl Editor {

    fn new(client: Client, socket: Option<Socket>, systems: &mut AppSystems) -> Self {
        Self {
            state: State {
                project: ProjectState::new(client),
                editor: EditorState::new(),
                renderer: None
            },
            docking: systems.prefs.get::<DockingLayoutPref>(),
            windows: pierro::WindowManager::new(),
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

        self.menu_bar(ui);

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

        self.state.editor.selection.begin_frame(ui.input().key_modifiers.contains(pierro::KeyModifiers::SHIFT));

        // Render the docking panels 
        if self.docking.render(ui, &mut self.state) {
            // Save the layout if it was modified
            systems.prefs.set::<DockingLayoutPref>(&self.docking);
        }

        // Render the windows on top
        self.state.editor.open_queued_windows(&mut self.windows);
        self.windows.render(ui, &mut self.state);

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
        
        // On load callbacks
        self.state.editor.process_on_load_callbacks(&self.state.project);
    }

}
