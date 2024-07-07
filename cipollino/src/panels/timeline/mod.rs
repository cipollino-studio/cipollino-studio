
use crate::app::{editor::EditorState, AppSystems};

use super::Panel;

#[derive(Default)]
pub struct Timeline {

}

impl Panel for Timeline {
    
    fn ui(&mut self, ui: &mut egui::Ui, state: &mut EditorState, systems: &mut AppSystems) {
        let Some(clip) = state.project.clips.get(state.open_clip) else {
            ui.centered_and_justified(|ui| {
                ui.label("No clip open");
            });
            return;
        };
        if !state.project.clips.is_loaded(state.open_clip) {
            ui.centered_and_justified(|ui| {
                ui.label("Clip not loaded");
            });
            return;
        }

        ui.heading(clip.name.value());
        for layer in clip.layers.iter_ref(&state.project.layers) {
            ui.label(layer.name.value());
        }

        let idx = clip.layers.len();
        let new_layer_idx = clip.layers.get_insertion_idx(idx);
        if ui.button("+").clicked() {
            state.client.add_layer(&mut state.project, state.open_clip, new_layer_idx, format!("Layer No. {}", idx), 1.0, false, false);
        }
    }

}
