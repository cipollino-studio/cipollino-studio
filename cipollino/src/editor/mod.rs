
use cipollino_project::{client::ProjectClient, project::{action::ActionManager, clip::Clip, layer::Layer, obj::ObjPtr, Project}};
use keybind::{Keybind, RedoKeybind, UndoKeybind};

use crate::{app::AppState, panels::PanelManager};

use crate::app::{prefs::UserPref, AppSystems};

mod menu_bar;
pub mod keybind;

// Utilities
pub mod playback;
pub mod frames;

struct PanelManagerPref;

impl UserPref for PanelManagerPref {
    type Type = PanelManager;

    fn default() -> Self::Type {
        PanelManager::new()
    }

    fn name() -> &'static str {
        "panel_manager"
    }
}

pub struct EditorState {
    pub project: Project,
    pub client: ProjectClient,
    pub actions: ActionManager,

    pub open_clip: ObjPtr<Clip>,
    pub active_layer: ObjPtr<Layer>,

    #[doc = "The current playback time, measured in samples"]
    pub playback_time: i64,
    pub playing: bool
}


pub struct Editor {
    state: EditorState,
    panel_manager: PanelManager,
    prev_time: f64
}

impl Editor {
    
    pub fn new(project: Project, client: ProjectClient, systems: &mut AppSystems) -> Self {
        Editor {
            state: EditorState {
                project,
                client,
                actions: ActionManager::new(),
                open_clip: ObjPtr::null(),
                active_layer: ObjPtr::null(),
                playback_time: 0,
                playing: false,
            }, 
            panel_manager: systems.prefs.get::<PanelManagerPref>(),
            prev_time: 0.0
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, systems: &mut AppSystems) -> Option<AppState> {
        let mut next_state = None;
        if let Err(msg) = self.state.client.update(&mut self.state.project) {
            next_state = Some(AppState::Error(msg));
        }

        // Load open clip
        self.state.client.load_clip(self.state.open_clip, &mut self.state.project);

        // Update active layer
        if let Some(clip) = self.state.project.clips.get(self.state.open_clip) {
            let reset_active_layer = match self.state.project.layers.get(self.state.active_layer) {
                Some(layer) => layer.clip.0 != self.state.open_clip,
                None => true,
            };
            if reset_active_layer {
                self.state.active_layer = clip.layers.iter().next().unwrap_or(ObjPtr::null());
            }
        } else {
            self.state.active_layer = ObjPtr::null();
        }

        let disabled = !self.state.client.has_keys();

        self.menu_bar(ctx, &mut next_state, disabled, systems);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_enabled(!disabled);
            self.panel_manager.update(ctx, &mut self.state, systems, disabled);
        });
 
        // Update playback time
        let time = ctx.input(|i| i.time);
        let dt = time - self.prev_time;
        self.state.update_playback(dt as f32, ctx); 

        // Keyboard shortcuts
        self.state.playback_shortcuts(ctx, systems.prefs);
        self.state.frames_shortcuts(ctx, systems.prefs);
        if RedoKeybind::consume(ctx, systems.prefs) {
            self.state.actions.redo(&mut self.state.project, &mut self.state.client);
        }
        if UndoKeybind::consume(ctx, systems.prefs) {
            self.state.actions.undo(&mut self.state.project, &mut self.state.client);
        }

        systems.prefs.set::<PanelManagerPref>(&self.panel_manager);

        self.prev_time = time;

        next_state 
    }

}
