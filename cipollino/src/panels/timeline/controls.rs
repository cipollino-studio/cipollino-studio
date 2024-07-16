
use crate::editor::EditorState;

use super::{Timeline, TimelineCommand, TimelineRenderInfo};


impl Timeline {

    fn controls(&mut self, ui: &mut egui::Ui, state: &EditorState, info: &mut TimelineRenderInfo) {
        if ui.button(egui_phosphor::regular::FILE_PLUS)
            .on_hover_text("Add Layer")
            .clicked() {
                info.commands.push(TimelineCommand::AddLayer);
        }
        if ui.button(egui_phosphor::regular::PLUS_CIRCLE)
            .on_hover_text("Add Frame")
            .clicked() {
                info.commands.push(TimelineCommand::AddFrame);
        }

        ui.add_space(20.0);

        if ui.button(egui_phosphor::regular::REWIND)
            .on_hover_text("Rewind to the start")
            .clicked() {

        }
        if ui.button(egui_phosphor::regular::CARET_CIRCLE_LEFT)
            .on_hover_text("Go to previous frame")
            .clicked() {

        }
        if ui.button(if state.playing { egui_phosphor::regular::PAUSE } else { egui_phosphor::regular::PLAY })
            .on_hover_text(if state.playing { "Pause" } else { "Play" })
            .clicked() {
                info.commands.push(TimelineCommand::TogglePlaying);
        }
        if ui.button(egui_phosphor::regular::CARET_CIRCLE_RIGHT)
            .on_hover_text("Go to next frame")
            .clicked() {

        }
    }
    
    pub(super) fn render_controls(&mut self, ui: &mut egui::Ui, state: &EditorState, info: &mut TimelineRenderInfo) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            self.controls(ui, state, info);
        });
    }

}
