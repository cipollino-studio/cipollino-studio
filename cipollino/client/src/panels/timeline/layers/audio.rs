
use alisa::Ptr;
use project::{Action, AudioLayer, DeleteAudioLayer};

use crate::{EditorState, ProjectState, TimelinePanel};

use super::LayerList;

impl TimelinePanel {

    fn audio_layer_context_menu(&mut self, ui: &mut pierro::UI, project: &ProjectState, editor: &EditorState, layer_ptr: Ptr<AudioLayer>) {
        if pierro::menu_button(ui, "Delete").mouse_clicked() {
            project.client.queue_action(Action::single(editor.action_context("Delete Audio Layer"), DeleteAudioLayer {
                ptr: layer_ptr,
            }));
        }
    }

    pub(super) fn render_audio_layer(&mut self, ui: &mut pierro::UI, project: &ProjectState, editor: &mut EditorState, render_list_idx: usize, depth: i32, audio: &AudioLayer, audio_ptr: Ptr<AudioLayer>) {

        ui.push_id_seed(&audio_ptr);
        let (layer_response, _) = pierro::container(ui, pierro::Size::fr(1.0), pierro::Size::px(Self::LAYER_HEIGHT), pierro::Layout::horizontal().align_center(), |ui| {
            pierro::container(ui, pierro::Size::fr(1.0).with_grow(1.0), pierro::Size::fr(1.0), pierro::Layout::horizontal().align_center().with_horizontal_overflow(), |ui| {
                pierro::h_spacing(ui, 2.0);
                pierro::icon(ui, pierro::icons::MUSIC_NOTES);
                pierro::h_spacing(ui, 2.0);

                self.layer_depth_spacing(ui, depth);

                self.renameable_layer_label(ui, project, &editor, &audio.name, audio_ptr); 
            });

            pierro::container(ui, pierro::Size::fit(), pierro::Size::fr(1.0), pierro::Layout::horizontal().align_center(), |ui| {

                // Mute/Unmute
                let mute_icon = if editor.muted_layers.contains(&audio_ptr) {
                    pierro::icons::SPEAKER_SLASH
                } else {
                    pierro::icons::SPEAKER_HIGH
                };
                if pierro::clickable_icon(ui, mute_icon).mouse_clicked() {
                    if !editor.muted_layers.remove(&audio_ptr) {
                        editor.muted_layers.insert(audio_ptr);
                    }
                }
                
                pierro::h_spacing(ui, 5.0);
            });

        });
        
        self.layer_dnd_source.source_without_cursor_icon(ui, &layer_response, || LayerList::single(audio_ptr));
        self.handle_layer_dropping(ui, &layer_response, render_list_idx);

        if layer_response.mouse_double_clicked() {
            self.start_rename(&audio.name, audio_ptr);
        }

        pierro::context_menu(ui, &layer_response, |ui| {
            self.audio_layer_context_menu(ui, project, &editor, audio_ptr); 
        });
    }

}
