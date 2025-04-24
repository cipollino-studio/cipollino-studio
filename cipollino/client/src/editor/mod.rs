
use std::path::PathBuf;

use alisa::Serializable;
use project::{deep_load_clip, Client, Frame, Ptr, Stroke};

use crate::splash::SplashScreen;
use crate::{AppState, AppSystems, DockingLayoutPref, EditorPanel, PanelContext};

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

mod presence;
pub use presence::*;

pub struct Editor {
    state: State,
    docking: pierro::DockingState<EditorPanel>,
    windows: pierro::WindowManager<WindowInstance>,
    socket: Option<Socket>,
    redraw_requests: u32 
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
            socket,
            redraw_requests: 0
        }
    }

    pub fn local(path: PathBuf, systems: &mut AppSystems) -> Option<Self> {
        Some(Self::new(Client::local(path)?, None, systems))
    }

    pub fn collab(socket: Socket, welcome_msg: &alisa::ABFValue, systems: &mut AppSystems) -> Option<Self> {
        Some(Self::new(Client::collab(welcome_msg)?, Some(socket), systems))
    }

    fn receive_message(msg: &alisa::ABFValue, state: &mut State) {
        let (msg_type, data) = match msg {
            alisa::ABFValue::NamedUnitEnum(name) => (name.as_str(), &alisa::ABFValue::PositiveInt(0)),
            alisa::ABFValue::NamedEnum(name, data) => (name.as_str(), &**data),
            _ => { return; }
        };

        if msg_type == "presence" {
            let Some(client_id) = data.get("client") else { return; };
            let Some(client_id) = client_id.as_u64() else { return; }; 
            let Some(data) = data.get("data") else { return; };
            let Some(data) = PresenceData::data_deserialize(data) else { return; };
            state.editor.other_clients.insert(client_id, data);
            return;
        }
        if msg_type == "disconnect" {
            let Some(client_id) = data.get("client") else { return; };
            let Some(client_id) = client_id.as_u64() else { return; }; 
            state.editor.other_clients.remove(&client_id);
            return;
        }

        state.project.client.receive_message(msg, &mut ());
    }

    pub fn tick(&mut self, ui: &mut pierro::UI, systems: &mut AppSystems, next_app_state: &mut Option<AppState>) {

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

        self.menu_bar(ui, next_app_state);
        self.use_shortcuts(ui, systems);

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

        // Collab
        if let Some(socket) = &mut self.socket {
            #[cfg(debug_assertions)]
            let send_messages = self.state.editor.send_messages;
            #[cfg(not(debug_assertions))]
            let send_messages = true;
            if send_messages {
                let to_send = self.state.project.client.take_messages();
                if !to_send.is_empty() {
                    socket.send(alisa::ABFValue::Array(to_send.into_iter().collect()));
                }
            }

            #[cfg(debug_assertions)]
            let receive_messages = self.state.editor.receive_messages;
            #[cfg(not(debug_assertions))]
            let receive_messages = true;
            if receive_messages {
                while let Some(msg) = socket.receive() {
                    if let Some(msgs) = msg.as_array() {
                        for submsg in msgs {
                            Self::receive_message(submsg, &mut self.state);
                        }
                    } else {
                        Self::receive_message(&msg, &mut self.state);
                    }
                    // Redraw the UI a few times to make sure everything gets updated
                    self.redraw_requests = 3;
                }
            }

            if !socket.has_signal() {
                socket.set_signal(ui.redraw_signal());
            }

            for updated_stroke in self.state.project.client.modified::<Stroke>() {
                // Invalidate cached meshes for updated strokes
                self.state.editor.stroke_mesh_cache.borrow_mut().remove(&updated_stroke);

                // If someone else we're collabing with modifies a stroke we selected, clear the selection to be safe
                if self.state.editor.selection.selected(updated_stroke) {
                    self.state.editor.selection.clear();
                }
            }
            self.state.project.client.clear_modified::<Stroke>();

            for updated_frame in self.state.project.client.modified::<Frame>() {
                // If someone else we're collabing with modifies a frame we selected, clear the selection to be safe
                if self.state.editor.selection.selected(updated_frame) {
                    self.state.editor.selection.clear();
                }
            }
            self.state.project.client.clear_modified::<Frame>();

            self.state.editor.presence.update(socket);

            if socket.closed() {
                let msg = "Collab server disconnected.".to_owned();
                *next_app_state = Some(AppState::SplashScreen(SplashScreen::new_with_error(msg)));
            }

        }

        // Update the project client
        self.state.project.tick(&self.state.editor);
        self.state.project.client.tick(&mut ());

        // Invalidate cached meshes for updated strokes
        for updated_stroke in self.state.project.client.modified() {
            self.state.editor.stroke_mesh_cache.borrow_mut().remove(&updated_stroke);
        }
        self.state.project.client.clear_modified::<Stroke>();

        // On load callbacks
        self.state.editor.process_on_load_callbacks(&self.state.project);

        // Pop the accent color style
        ui.pop_style();

        if self.redraw_requests > 0 {
            self.redraw_requests -= 1;
            ui.request_redraw();
        }
        self.state.project.client.clear_all_modified();
    }

}
