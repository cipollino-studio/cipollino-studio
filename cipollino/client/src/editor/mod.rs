
use std::path::PathBuf;

use alisa::Children;
use project::{deep_load_clip, Client, Fill, Frame, Message, Ptr, Stroke, WelcomeMessage, PROTOCOL_VERSION};

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

mod window;
pub use window::*;

mod settings;
pub use settings::*;

mod presence;
pub use presence::*;

mod clipboard;
pub use clipboard::*;

mod layer_render_list;
pub use layer_render_list::*;

mod scene_render_list;
pub use scene_render_list::*;

mod mesh_cache;
pub use mesh_cache::*;

pub struct Editor {
    state: State,
    docking: pierro::DockingState<EditorPanel>,
    windows: pierro::WindowManager<WindowInstance>,
    socket: Option<Socket>,
    redraw_requests: u32 
}

impl Editor {

    fn new(client: Client, socket: Option<Socket>, systems: &mut AppSystems) -> Self {
        let mut editor = Self {
            state: State {
                project: ProjectState::new(client),
                editor: EditorState::new(systems),
                renderer: None
            },
            docking: systems.prefs.get::<DockingLayoutPref>(),
            windows: pierro::WindowManager::new(),
            socket,
            redraw_requests: 3
        };

        let client = &editor.state.project.client;
        if client.folders.is_empty() && client.clips.n_children() == 1 {
            if let Some(clip_ptr) = client.clips.iter().next() {
                editor.state.editor.open_clip(clip_ptr.ptr());
            }
        }

        editor
    }

    pub fn local(path: PathBuf, systems: &mut AppSystems) -> Option<Self> {
        Some(Self::new(Client::local(path)?, None, systems))
    }

    pub fn collab(socket: Socket, welcome_msg: &alisa::ABFValue, systems: &mut AppSystems) -> Result<Self, String> {
        let welcome_msg = alisa::deserialize::<WelcomeMessage>(welcome_msg).ok_or("Invalid server protocol.".to_owned())?;
        let mut editor = Self::new(Client::collab(&welcome_msg.collab).ok_or("Invalid server protocol.".to_owned())?, Some(socket), systems);
        editor.state.editor.other_clients = welcome_msg.presence.into_iter().collect();

        if PROTOCOL_VERSION > welcome_msg.version {
            return Err("Outdated server version. Update the server software.".to_owned());
        }
        if PROTOCOL_VERSION < welcome_msg.version {
            return Err("Outdated client version. Install the latest version of Cipollino.".to_owned());
        }
    
        Ok(editor)
    }

    fn receive_message(msg: &Message, state: &mut State) {

        match msg {
            Message::Collab(msg) => {
                state.project.client.receive_message(msg);
            },
            Message::PresenceUpdate(client_id, presence_data) => {
                state.editor.other_clients.insert(*client_id, presence_data.clone());
            },
            Message::Disconnect(client_id) => {
                state.editor.other_clients.remove(&client_id);
            },
            _ => {}
        }

    }

    pub fn tick(&mut self, ui: &mut pierro::UI, systems: &mut AppSystems, next_app_state: &mut Option<AppState>) {

        // Set the accent color
        ui.push_style::<pierro::theme::AccentColor>(systems.prefs.get::<AccentColor>());

        // Load the currently open clip if it's not open
        if let Some(clip) = self.state.project.client.get(self.state.editor.open_clip) {
            if let Some(clip_inner) = self.state.project.client.get(clip.inner) {
                self.state.editor.tick_playback(ui, systems, clip_inner);
            } else {
                deep_load_clip(self.state.editor.open_clip, &self.state.project.client);
            }
        }

        if let Some(new_selection) = self.state.editor.next_selection.take() {
            self.state.editor.selection.replace(new_selection);
        }

        // Removed locked objects from the selection 
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

        // Calculate layer and scene render lists
        let (layer_render_list, scene_render_list) = self.state.project.client.get(self.state.editor.open_clip).and_then(|clip| {
            self.state.project.client.get(clip.inner)
        }).map(|inner| (
            LayerRenderList::make(&self.state.project.client, &self.state.editor, inner),
            SceneRenderList::make(&self.state.project.client, &self.state.editor, inner, inner.frame_idx(self.state.editor.time)) 
        )).unzip();

        // Recalculate any necessary meshes
        if let Some(render_list) = &scene_render_list {
            self.state.editor.mesh_cache.calculate(&render_list, &self.state.project.client, ui.wgpu_device());
        }

        self.state.editor.use_shortcuts(&self.state.project, layer_render_list.as_ref(), scene_render_list.as_ref(), ui, systems);

        self.state.editor.selection.begin_frame(ui.input().key_modifiers.contains(pierro::KeyModifiers::SHIFT));

        // Render the docking panels 
        let mut panel_context = PanelContext {
            editor: &mut self.state.editor,
            project: &self.state.project,
            systems,
            renderer: &mut self.state.renderer,
            layer_render_list: layer_render_list.as_ref(),
            scene_render_list: scene_render_list.as_ref()
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
            renderer: &mut self.state.renderer,
            layer_render_list: layer_render_list.as_ref(),
            scene_render_list: scene_render_list.as_ref()
        };
        self.windows.render(ui, &mut panel_context);

        self.state.editor.selection.end_frame(ui.input().l_mouse.clicked() || ui.input().l_mouse.drag_started());

        if let Some(layers) = layer_render_list {
            self.state.editor.tick_audio_playback(systems, &self.state.project, &layers);
        } 

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
                    socket.send_data(
                        alisa::ABFValue::Array(
                            to_send.into_iter()
                                .map(|msg| alisa::serialize(&Message::Collab(msg)))
                                .collect()
                        )
                    );
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
                            let Some(submsg) = alisa::deserialize::<Message>(submsg) else {
                                continue;
                            };
                            Self::receive_message(&submsg, &mut self.state);
                        }
                    } else {
                        if let Some(msg) = alisa::deserialize::<Message>(&msg) {
                            Self::receive_message(&msg, &mut self.state);
                        }
                    }
                    // Redraw the UI a few times to make sure everything gets updated
                    self.redraw_requests = 3;
                }
            }

            if !socket.has_signal() {
                socket.set_signal(ui.redraw_signal());
            }

            for updated_stroke in self.state.project.client.modified::<Stroke>() {
                // If someone else we're collabing with modifies a stroke we selected, clear the selection to be safe
                if self.state.editor.selection.selected(updated_stroke) {
                    self.state.editor.selection.clear();
                }
            }
            for updated_fill in self.state.project.client.modified::<Fill>() {
                // If someone else we're collabing with modifies a fill we selected, clear the selection to be safe
                if self.state.editor.selection.selected(updated_fill) {
                    self.state.editor.selection.clear();
                }
            }
            for updated_frame in self.state.project.client.modified::<Frame>() {
                // If someone else we're collabing with modifies a frame we selected, clear the selection to be safe
                if self.state.editor.selection.selected(updated_frame) {
                    self.state.editor.selection.clear();
                }
            }

            // Invalidate the mesh cache of any modified objects
            self.state.editor.mesh_cache.invalidate(&self.state.project.client);

            self.state.editor.presence.update(socket);

            if socket.closed() {
                let msg = "Collab server disconnected.".to_owned();
                *next_app_state = Some(AppState::SplashScreen(SplashScreen::new_with_error(msg)));
            }

        }

        // Update the project client
        self.state.project.tick(&self.state.editor);
        self.state.project.client.tick();

        // Invalidate cached meshes
        self.state.editor.mesh_cache.invalidate(&self.state.project.client);

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
