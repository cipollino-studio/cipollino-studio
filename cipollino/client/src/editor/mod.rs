
use std::path::PathBuf;

use project::alisa::rmpv;
use project::{deep_load_clip, Client, Frame, Ptr, Stroke};

use crate::{AppSystems, DockingLayoutPref, EditorPanel, PanelContext};

mod socket;
pub use socket::*;

mod state;
pub use state::*;

mod undo_redo;

mod selection;
pub use selection::*;

mod shortcuts;
pub use shortcuts::*;

mod menu_bar;

mod export;
use export::*;

mod window;
pub use window::*;

mod settings;
pub use settings::*;

pub struct Editor {
    state: State,
    docking: pierro::DockingState<EditorPanel>,
    windows: pierro::WindowManager<WindowInstance>,
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

        // Set the accent color
        ui.push_style::<pierro::theme::AccentColor>(systems.prefs.get::<AccentColor>());

        // Load the currently open clip if it's not open
        if let Some(clip) = self.state.project.client.get(self.state.editor.open_clip) {
            if let Some(clip_inner) = self.state.project.client.get(clip.inner) {
                self.state.editor.tick_playback(ui, clip_inner);
            } else {
                deep_load_clip(self.state.editor.open_clip, &self.state.project.client);
            }
        }

        // Removed locked objects from the seletion 
        self.state.editor.selection.retain(|frame: Ptr<Frame>| {
            let Some(frame) = self.state.project.client.get(frame) else { return  false; };
            !self.state.editor.locked_layers.contains(&frame.layer)
        });
        self.state.editor.selection.retain(|stroke: Ptr<Stroke>| {
            let Some(stroke) = self.state.project.client.get(stroke) else { return false; };
            let Some(frame) = self.state.project.client.get(stroke.frame) else { return false; };
            !self.state.editor.locked_layers.contains(&frame.layer) && !self.state.editor.hidden_layers.contains(&frame.layer)
        });

        self.menu_bar(ui);
        self.use_shortcuts(ui, systems);

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
        let mut panel_context = PanelContext {
            editor: &mut self.state.editor,
            project: &self.state.project,
            systems,
            renderer: &mut self.state.renderer
        };
        if self.docking.render(ui, &mut panel_context) {
            // Save the layout if it was modified
            systems.prefs.set::<DockingLayoutPref>(&self.docking);
        }

        // Render the windows on top
        self.state.editor.open_queued_windows(&mut self.windows);
        let mut panel_context = PanelContext {
            editor: &mut self.state.editor,
            project: &self.state.project,
            systems,
            renderer: &mut self.state.renderer
        };
        self.windows.render(ui, &mut panel_context);

        self.state.editor.selection.end_frame(ui.input().l_mouse.clicked() || ui.input().l_mouse.drag_started());

        self.tick_undo_redo(); 

        self.state.editor.preview.end_frame();

        // Update the project client
        self.state.project.tick(&self.state.editor);
        self.state.project.client.tick(&mut ());

        // Invalidate cached meshes for updated strokes
        for updated_stroke in self.state.project.client.modified() {
            self.state.editor.stroke_mesh_cache.borrow_mut().remove(&updated_stroke);
        }
        
        // On load callbacks
        self.state.editor.process_on_load_callbacks(&self.state.project);

        // Pop the accent color style
        ui.pop_style();
    }

}
