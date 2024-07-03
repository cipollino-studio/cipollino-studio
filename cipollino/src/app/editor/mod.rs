
use cipollino_project::{client::ProjectClient, project::{action::ActionManager, Project}};
use keybind::{Keybind, RedoKeybind, UndoKeybind};

use crate::{app::AppState, panels::PanelManager};

use super::{prefs::UserPref, AppSystems};

mod menu_bar;
pub mod keybind;

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
    pub actions: ActionManager 
}

pub struct Editor {
    state: EditorState,
    panel_manager: PanelManager
}

impl Editor {
    
    pub fn new(project: Project, client: ProjectClient, systems: &mut AppSystems) -> Self {
        Editor {
            state: EditorState {
                project,
                client,
                actions: ActionManager::new()
            }, 
            panel_manager: systems.prefs.get::<PanelManagerPref>()
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, systems: &mut AppSystems) -> Option<AppState> {
        let mut next_state = None;
        if let Err(msg) = self.state.client.update(&mut self.state.project) {
            next_state = Some(AppState::Error(msg));
        }

        let disabled = !self.state.client.has_keys();

        self.menu_bar(ctx, &mut next_state, disabled, systems);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_enabled(!disabled);
            self.panel_manager.update(ctx, &mut self.state, systems, disabled);
        });

        if RedoKeybind::consume(ctx, systems.prefs) {
            self.state.actions.redo(&mut self.state.project, &mut self.state.client);
        }
        if UndoKeybind::consume(ctx, systems.prefs) {
            self.state.actions.undo(&mut self.state.project, &mut self.state.client);
        }

        systems.prefs.set::<PanelManagerPref>(&self.panel_manager);

        next_state 
    }

}
