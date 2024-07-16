
use cipollino_project::{crdt::fractional_index::FractionalIndex, project::action::Action};

use crate::app::prefs::UserPrefs;

use super::{keybind::{Keybind, NewFrameKeybind}, EditorState};

impl EditorState {

    pub fn create_frame(&mut self) {
        let Some(layer) = self.project.layers.get(self.active_layer) else { return; };
        if layer.find_frame_exactly_at(&self.project.frames, self.playback_frame()).is_some() {
            return;
        }
        let mut action = Action::new();
        let frame = self.playback_frame();
        self.client.add_frame(&mut self.project, self.active_layer, FractionalIndex::half(), frame, &mut action);
        self.actions.push_action(action);
    }

    pub fn frames_shortcuts(&mut self, ctx: &egui::Context, prefs: &mut UserPrefs) {
        if NewFrameKeybind::consume(ctx, prefs) {
            self.create_frame();
        }
    }

}
